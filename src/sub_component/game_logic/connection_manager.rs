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

struct Channel<T> {
    tx:Sender<T>,
    rx:Receiver<T>,
}

enum EventCtoS {
    Accept(TcpStream),
    Message((ConnectionID,Vec<u8>)),
}

enum EventStoC {
    Message((ConnectionID,Vec<u8>)),
}

pub struct ConnectionManager {
    local_addr:String,
    ctos_channel:Channel<EventCtoS>,
    stoc_channel:Channel<EventStoC>,
    connections:HashMap<ConnectionID,TcpStream>,
}

impl ConnectionManager {
    pub fn new(addr:String) -> ConnectionManager {
        let ctos = channel();
        let stoc = channel();
        ConnectionManager {
            local_addr:addr,
            ctos_channel:Channel::<EventCtoS>{tx:ctos.0, rx:ctos.1},
            stoc_channel:Channel::<EventStoC>{tx:stoc.0, rx:stoc.1},
            connections:HashMap::new(),
        }
    }
    pub fn send(&mut self, conn_id:ConnectionID, data:Vec<u8>) {
        self.stoc_channel.tx.send(EventStoC::Message((conn_id,data)));
    }

    fn start_listener(&mut self) {
        match TcpListener::bind(self.local_addr.as_str()) {
            Ok(listener) => {
                let tx = self.ctos_channel.tx.clone();
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

    fn start_stoc_event_processor(&self) {
        thread::spawn(move|| {

        });
    }
}
impl SubComponent for ConnectionManager {
    fn start(&mut self) {
        self.start_listener();
        self.start_stoc_event_processor();
    }

    fn update(&mut self) {
        match self.ctos_channel.rx.recv() {
            Ok(EventCtoS::Accept(stream)) => {
                for id in 0..i32::max_value() {
                    if !self.connections.contains_key(&id) {
                        self.connections.insert(id, stream.try_clone().unwrap());
                    }
                }
                panic!("connection max");
            },
            Ok(EventCtoS::Message(msg)) => {
                // proccess msg;
            },
            Err(e) => {

            }
        }
    }
}
