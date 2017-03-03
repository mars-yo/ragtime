use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::mpsc::{Sender,Receiver};

use entity_component::component::*;

pub struct ConnectionManager {
    //ゲームロジック側でSenderを持つのがよいかも
    channel:(Sender<TcpStream>,Receiver<TcpStream>),
    connections:HashMap<i32,TcpStream>,
}

impl ConnectionManager {
    pub fn new() {

    }
    pub fn send(&mut self, conn_id:i32) {

    }
}
impl SubComponent for ConnectionManager {
    fn start(&mut self) {

        //待ち受けスレッドにする
        match TcpListener::bind("127.0.0.1:53000") {
            Ok(listener) => {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            if let Ok(str) = stream.try_clone() {
                                self.channel.0.send(str);
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
            },
            Err(e) => {
                println!("{}", e);
            }
        }

    }

    fn update(&mut self) {
        //別スレッドで各コネクションからくるメッセージを受信
        //送信用チャンネルからデータをとって送信
        let stream = self.channel.1.recv().unwrap();
        for id in 0..i32::max_value() {
            if !self.connections.contains_key(&id) {
                self.connections.insert(id, stream.try_clone().unwrap());
            }
        }
        panic!("connection max");
    }

}
