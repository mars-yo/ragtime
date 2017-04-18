extern crate byteorder;

use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::io::{Write, Read, BufReader, BufRead};
use self::byteorder::{BigEndian, ByteOrder};
use entity_component::component::*;

pub type ConnectionID = u32;

pub trait Message {
    fn new() -> Self;
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool;
}
pub trait MessageHandler<T: Message> {
    fn on_message(&mut self, id: ConnectionID, msg: &T);
}

struct Connection<T: Message, U: MessageHandler<T>> {
    id: ConnectionID,
    send_stream: TcpStream,
    recv_buffer: BufReader<TcpStream>,
    message: T,
    message_handler: Weak<RefCell<U>>,
}

impl<T: Message, U: MessageHandler<T>> Connection<T, U> {
    fn new(id: ConnectionID, stream: TcpStream) -> Connection<T, U> {
        let buf = BufReader::with_capacity(1024, stream.try_clone().unwrap());
        Connection::<T, U> {
            id: id,
            send_stream: stream,
            recv_buffer: buf,
            message: Message::new(),
            message_handler: Weak::new(),
        }
    }
    fn recv(&mut self) {
        if self.message.read_from(&mut self.recv_buffer) {
            if let Some(handler) = self.message_handler.upgrade() {
                handler.borrow_mut().on_message(self.id, &self.message);
            }
        }
    }
    fn send(&mut self, data: &[u8]) {
        self.send_stream.write_all(data);
    }
    fn set_message_handler(&mut self, handler: Weak<RefCell<U>>) {
        self.message_handler = handler;
    }
}


pub struct ConnectionManager<T: Message, U: MessageHandler<T>> {
    next_conn_id: ConnectionID,
    listener: TcpListener,
    connections: HashMap<ConnectionID, Connection<T, U>>,
    default_message_handler: Weak<RefCell<U>>,
}

impl<T: Message, U: MessageHandler<T>> ConnectionManager<T, U> {
    pub fn new(addr: String) -> ConnectionManager<T, U> {
        let listener = TcpListener::bind(addr.as_str()).expect("listener bind error");
        listener
            .set_nonblocking(true)
            .expect("listener can not set nonblocking");
        ConnectionManager::<T, U> {
            next_conn_id: 0,
            listener: listener,
            connections: HashMap::new(),
            default_message_handler: Weak::new(),
        }
    }
    pub fn send_to(&mut self, conn_id: ConnectionID, data: Vec<u8>) {
        if let Some(val) = self.connections.get_mut(&conn_id) {
            val.send(data.as_slice());
        }
    }
    pub fn set_message_handler(&mut self, conn_id: ConnectionID, handler: Weak<RefCell<U>>) {
        if let Some(val) = self.connections.get_mut(&conn_id) {
            val.set_message_handler(handler);
        }
    }
}

impl<T: Message, U: MessageHandler<T>> SubComponent for ConnectionManager<T, U> {
    fn start(&mut self) {}

    fn update(&mut self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true);
                let mut conn = Connection::new(self.next_conn_id, stream);
                conn.set_message_handler(self.default_message_handler.clone());
                self.connections.insert(self.next_conn_id, conn);
                println!("new connection {} {}", self.next_conn_id, addr);
                self.next_conn_id += 1;
            }
            Err(e) => {
                println!("no new connection");
            }
        }

        for (conn_id, conn) in self.connections.iter_mut() {
            conn.recv();
        }
    }
}
