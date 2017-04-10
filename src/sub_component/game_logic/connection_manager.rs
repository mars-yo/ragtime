extern crate byteorder;

use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::{HashMap,VecDeque};
use std::sync::mpsc::{Sender,Receiver,channel};
use std::rc::{Weak,Rc};
use std::cell::RefCell;
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::io::{Write,Read,BufReader,BufRead};
use self::byteorder::{BigEndian,ByteOrder};
use entity_component::component::*;

pub type ConnectionID = u32;

pub trait Message {
    fn new() -> Self;
    fn read_from<T:BufRead>(&mut self, reader:&mut T) -> bool;
}
pub trait HandleMessage<T:Message> {
    fn on_message(&mut self, msg:&T);
}

struct Connection<T:Message, U:HandleMessage<T>> {
    send_stream:TcpStream,
    recv_buffer:BufReader<TcpStream>,
    message:T,
    message_handler:Weak<RefCell<U>>,
}

impl<T:Message, U:HandleMessage<T>> Connection<T,U> {
    fn new(stream:TcpStream) -> Connection<T,U> {
        let buf = BufReader::with_capacity(1024, stream.try_clone().unwrap());
        Connection::<T,U> {
            send_stream:stream,
            recv_buffer:buf,
            message:Message::new(),
            message_handler:Weak::new(),
        }
    }
    fn recv(&mut self) {
        if self.message.read_from(&mut self.recv_buffer) {
            if let Some(handler) = self.message_handler.upgrade() {
                handler.borrow_mut().on_message(&self.message);
            }
        }
        // let mut raw:[u8;4] = [0;4];
        // let mut consumed = false;
        // match self.recv_buffer.fill_buf() {
        //     Err(e) => println!("error {} {} {}", file!(), line!(), e),
        //     Ok(data) => {
        //         println!("data len {}", data.len());
        //         if data.len() >= 4 {
        //             for i in 0..4 {
        //                 raw[i] = *data.get(i).unwrap();
        //             }
        //             consumed = true;
        //         }
        //     }
        // }
        // if consumed {
        //     self.recv_header.body_len = BigEndian::read_i32(&raw[..]);
        //     self.recv_buffer.consume(4);
        //     println!("read {}", self.recv_header.body_len);
        // }
    }
    fn send(&mut self, data:&[u8]) {
        self.send_stream.write_all(data);
    }
}


pub struct ConnectionManager<T:Message, U:HandleMessage<T>> {
    next_conn_id:ConnectionID,
    listener:TcpListener,
    connections:HashMap<ConnectionID, Connection<T,U>>,
}

impl<T:Message, U:HandleMessage<T>> ConnectionManager<T,U> {
    pub fn new(addr:String) -> ConnectionManager<T,U> {
        let listener = TcpListener::bind(addr.as_str()).expect("listener bind error");
        listener.set_nonblocking(true).expect("listener can not set nonblocking");
        ConnectionManager::<T,U> {
            next_conn_id:0,
            listener:listener,
            connections:HashMap::new(),
        }
    }
    pub fn send_to(&mut self, conn_id:ConnectionID, data:Vec<u8>) {
        if let Some(val) = self.connections.get_mut(&conn_id) {
            val.send(data.as_slice());
        }
    }
}

impl<T:Message,U:HandleMessage<T>> SubComponent for ConnectionManager<T,U> {
    fn start(&mut self) {
    }

    fn update(&mut self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true);
                self.connections.insert(self.next_conn_id, Connection::new(stream));
                println!("new connection {} {}", self.next_conn_id, addr);
                self.next_conn_id += 1;
            },
            Err(e) => {
                println!("no new connection");
            }
        }

        for (conn_id, conn) in self.connections.iter_mut() {

            // let ref mut stream = str_buf.0;
            // let ref mut buffer = str_buf.1;
            // if let Ok(Some(e)) = stream.take_error() {
            //     println!("error on socket {}", e);
            // }
            // let completed = conn.recv();
            // if completed {
            //     (self.msg_handler)(*conn_id, &conn.message);
            // }
        }

        // for each connection recv data
    }
}
