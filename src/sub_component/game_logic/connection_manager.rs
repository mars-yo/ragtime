use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::{HashMap,VecDeque};
use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::io::Write;
use entity_component::component::*;

type ConnectionID = i32;

enum EventCtoS {
    Accept(TcpStream),
}

enum EventStoC {
    SendAll(Vec<u8>),
    SendTo((ConnectionID,Vec<u8>)),
    SendExcept((ConnectionID,Vec<u8>)),
}

pub struct ConnectionManager {
    local_addr:String,
    ctos_channel_rx:Receiver<EventCtoS>,
    stoc_channel_tx:Sender<EventStoC>,
    connections:HashMap<ConnectionID,TcpStream>,
}

impl ConnectionManager {
    pub fn new(addr:String) -> ConnectionManager {
        let ctos = channel();
        let stoc = channel();
        ConnectionManager {
            local_addr:addr,
            ctos_channel_rx:ctos.1,
            stoc_channel_tx:stoc.0,
            connections:HashMap::new(),
        }
    }
    pub fn send_to(&mut self, conn_id:ConnectionID, data:Vec<u8>) {
        self.stoc_channel_tx.send(EventStoC::SendTo((conn_id,data)));
    }

    fn start_listener(&mut self) {
        match TcpListener::bind(self.local_addr.as_str()) {
            Ok(listener) => {
                let (tx,rx) = channel();
                self.ctos_channel_rx = rx;
                thread::spawn(move|| {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                if let Ok(str) = stream.try_clone() {
                                    tx.send(EventCtoS::Accept(str));
                                    break;
                                }
                                else {
                                    break;
                                }
                            },
                            Err(e) => {

                            }
                        }
                    }
                });
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn start_stoc_event_processor(&mut self, rx:Receiver<EventStoC>) {
        thread::spawn(move|| {
            loop {
                match rx.recv().unwrap() {
                    EventStoC::SendAll(data) => {

                    },
                    EventStoC::SendExcept((conn_id,data)) => {

                    },
                    EventStoC::SendTo((conn_id,data)) => {

                    }
                }
            }
        });
    }
}
impl SubComponent for ConnectionManager {
    fn start(&mut self) {
        self.start_listener();
        let chan = channel();
        self.stoc_channel_tx = chan.0;
        self.start_stoc_event_processor(chan.1);
    }

    fn update(&mut self) {
        match self.ctos_channel_rx.try_recv() {
            Ok(EventCtoS::Accept(stream)) => {
                for id in 0..i32::max_value() {
                    if !self.connections.contains_key(&id) {
                        self.connections.insert(id, stream.try_clone().unwrap());
                    }
                }
                panic!("connection max");
            },
            Err(e) => {

            }
        }

        // for each connection recv data
    }
}
