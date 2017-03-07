use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;

use entity_component::component::*;

type ConnectionID = i32;

pub struct ConnectionManager {
    //ゲームロジック側でSenderを持つのがよいかも
    local_addr:String,
    channel:(Sender<TcpStream>,Receiver<TcpStream>),
    connections:HashMap<ConnectionID,TcpStream>,
}

impl ConnectionManager {
    pub fn new(addr:String) -> ConnectionManager {
        ConnectionManager {
            local_addr:addr,
            channel:channel(),
            connections:HashMap::new(),
        }
    }
    pub fn send(&mut self, conn_id:ConnectionID) {

    }
}
impl SubComponent for ConnectionManager {
    fn start(&mut self) {

        match TcpListener::bind(self.local_addr.as_str()) {
            Ok(listener) => {
                let tx = self.channel.0.clone();
                thread::spawn(move|| {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                if let Ok(str) = stream.try_clone() {
                                    tx.send(str);
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

    fn update(&mut self) {
        //送信用チャンネルからデータをとって送信
        match self.channel.1.try_recv() {
            Ok(stream) => {
                for id in 0..i32::max_value() {
                    if !self.connections.contains_key(&id) {
                        self.connections.insert(id, stream.try_clone().unwrap());
                    }
                }
                panic!("connection max");
            },
            Err(e) => {
                //err
            }
        }
    }

}
