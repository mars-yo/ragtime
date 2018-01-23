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
use std::mem;
use std::fmt::Debug;
use self::byteorder::{BigEndian, ByteOrder};

pub type ConnectionID = u32;

pub trait Serialize {
    fn new() -> Self;
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool;
}

struct Connection<T: Serialize+Debug> {
    id: ConnectionID,
    send_stream: TcpStream,
    recv_buffer: BufReader<TcpStream>,
    message: T,
}

impl<T: Serialize+Debug> Connection<T> {
    fn new(id: ConnectionID, stream: TcpStream) -> Connection<T> {
        let buf = BufReader::with_capacity(1024, stream.try_clone().unwrap());
        Connection::<T> {
            id: id,
            send_stream: stream,
            recv_buffer: buf,
            message: Serialize::new(),
        }
    }
    fn recv(&mut self) -> Option<T> {
        if self.message.read_from(&mut self.recv_buffer) {
            let mut msg = T::new();
            mem::swap(&mut msg, &mut self.message);
            return Some(msg);
        }
        None
    }
    fn send(&mut self, data: &[u8]) {
        self.send_stream.write_all(data);
    }
}

pub struct ConnectionManager<T: Serialize+Debug> {
    next_conn_id: ConnectionID,
    listener: TcpListener,
    connections: HashMap<ConnectionID, Connection<T>>,
}

impl<T: Serialize+Debug> ConnectionManager<T> {
    pub fn new(addr: String) -> ConnectionManager<T> {
        let listener = TcpListener::bind(addr.as_str()).expect("listener bind error");
        listener
            .set_nonblocking(true)
            .expect("listener can not set nonblocking");
        ConnectionManager::<T> {
            next_conn_id: 0,
            listener: listener,
            connections: HashMap::new(),
//            default_recv_msg_chan_tx:default_recv_msg_chan_tx,
        }
    }
    pub fn send_to(&mut self, conn_id: ConnectionID, data: Vec<u8>) {
        if let Some(val) = self.connections.get_mut(&conn_id) {
            val.send(data.as_slice());
        }
    }
    pub fn listen(&mut self) -> Option<ConnectionID> {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true);
                let mut conn = Connection::new(self.next_conn_id, stream);
                self.connections.insert(self.next_conn_id, conn);
                println!("new connection {} {}", self.next_conn_id, addr);
                let conn_id = self.next_conn_id;
                self.next_conn_id += 1;
                return Some(conn_id);
            }
            Err(e) => {
                // println!("no new connection");
            }
        }
        None
    }
    pub fn recv<F>(&mut self, f:F) where F:Fn(ConnectionID,T) -> () {
        for (conn_id, conn) in self.connections.iter_mut() {
            if let Some(msg) = conn.recv() {
                f(*conn_id, msg);
            }
        }
    }
}
