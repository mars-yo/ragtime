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
use self::byteorder::{BigEndian, ByteOrder};

//デフォルトハンドラをどうしよう、FnMutのクローンできない？

pub type ConnectionID = u32;
pub type MsgOnChannel<T> = Box<(ConnectionID,T)>;
pub type MsgChanTx<T> = Sender<MsgOnChannel<T>>;
pub type MsgChanRx<T> = Receiver<MsgOnChannel<T>>;

pub trait Message {
    fn new() -> Self;
    fn read_from<T: BufRead>(&mut self, reader: &mut T) -> bool;
}

struct Connection<T: Message> {
    id: ConnectionID,
    send_stream: TcpStream,
    recv_buffer: BufReader<TcpStream>,
    message: T,
    s2c_chan_rx: MsgChanRx<T>,
    c2s_chan_tx: MsgChanTx<T>,
}

impl<T: Message> Connection<T> {
    fn new(id: ConnectionID, stream: TcpStream, s2c_rx: MsgChanRx<T>, c2s_tx: MsgChanTx<T>) -> Connection<T> {
        let buf = BufReader::with_capacity(1024, stream.try_clone().unwrap());
        Connection::<T> {
            id: id,
            send_stream: stream,
            recv_buffer: buf,
            message: Message::new(),
            s2c_chan_rx: s2c_rx,
            c2s_chan_tx: c2s_tx,
        }
    }
    fn recv(&mut self) {
        if self.message.read_from(&mut self.recv_buffer) {
            let mut msg = T::new();
            mem::swap(&mut msg, &mut self.message);
            self.c2s_chan_tx.send(Box::new((self.id,msg)));
        }
    }
    fn send(&mut self, data: &[u8]) {
        self.send_stream.write_all(data);
    }
//     fn s2c_chan_tx(&self) -> MsgChanTx<T> {
//         self.s2c_chan_tx.clone()
//     }
//     fn set_c2s_chan_tx(&mut self, tx: MsgChanTx<T>) {
//         self.c2s_chan_tx = tx;
//     }
}

pub struct ConnectionManager<T: Message> {
    next_conn_id: ConnectionID,
    listener: TcpListener,
    connections: HashMap<ConnectionID, Connection<T>>,
//    default_recv_msg_chan_tx: Sender<MessageOnChannel<T>>,
}

impl<T: Message> ConnectionManager<T> {
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
//     pub fn send_msg_chan(&self, conn_id:ConnectionID) -> Option<Sender<MessageOnChannel<T>>> {
//         if let Some(val) = self.connections.get(&conn_id) {
//             return Some(val.send_msg_chan_tx())
//         }
//         None
//     }
//     pub fn set_recv_message_chan(&mut self, conn_id: ConnectionID, tx: Sender<MessageOnChannel<T>>) {
//         if let Some(val) = self.connections.get_mut(&conn_id) {
//             val.set_recv_msg_chan_tx(tx);
//         }
//     }
    pub fn poll(&mut self) -> Option<(MsgChanTx<T>,MsgChanRx<T>)> {
        for (conn_id, conn) in self.connections.iter_mut() {
            conn.recv();
        }
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true);
                let (s2c_tx, s2c_rx) = channel();
                let (c2s_tx, c2s_rx) = channel();
                let mut conn = Connection::new(self.next_conn_id, stream, s2c_rx, c2s_tx);
                self.connections.insert(self.next_conn_id, conn);
                println!("new connection {} {}", self.next_conn_id, addr);
                self.next_conn_id += 1;
                return Some((s2c_tx, c2s_rx))
            }
            Err(e) => {
                // println!("no new connection");
            }
        }
        None
    }
}
