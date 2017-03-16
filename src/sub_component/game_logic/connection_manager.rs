use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::{HashMap,VecDeque};
use std::sync::mpsc::{Sender,Receiver,channel};
use std::sync::Arc;
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::io::{Write,Read};
use entity_component::component::*;

type ConnectionID = u32;

enum EventCtoS {
    Accept(TcpStream),
}

enum EventStoC {
    SendTo((TcpStream,Vec<u8>)),
}

pub struct ConnectionManager {
    next_conn_id:ConnectionID,
    listener:TcpListener,
    connections:HashMap<ConnectionID,TcpStream>,
}

impl ConnectionManager {
    pub fn new(addr:String) -> ConnectionManager {
        let listener = TcpListener::bind(addr.as_str()).expect("listener bind error");
        listener.set_nonblocking(true).expect("listener can not set nonblocking");
        ConnectionManager {
            next_conn_id:0,
            listener:listener,
            connections:HashMap::new(),
        }
    }
    pub fn send_to(&mut self, conn_id:ConnectionID, data:Vec<u8>) {
        if let Some(conn) = self.connections.get_mut(&conn_id) {
            conn.write_all(data.as_slice());
        }
    }
}

impl SubComponent for ConnectionManager {
    fn start(&mut self) {
    }

    fn update(&mut self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true);
                self.connections.insert(self.next_conn_id, stream);
                println!("new connection {} {}", self.next_conn_id, addr);
                self.next_conn_id += 1;
            },
            Err(e) => {
                println!("no new connection");
            }
        }

        for (conn_id, stream) in self.connections.iter_mut() {

            if let Ok(Some(e)) = stream.take_error() {
                println!("error on socket {}", e);
            }

            let mut header:[u8;1] = [0;1];
            match stream.read_exact(&mut header) {
                Err(e) => println!("error {} {} {}", file!(), line!(), e),
                _ => {}
            }
            println!("read {:?}", header);
        }

        // for each connection recv data
    }
}
