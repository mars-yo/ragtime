#[macro_use]
extern crate ragtime;
extern crate ws;

//mod components;
mod game;

use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use ragtime::room;
//use game_scene::room::*;

struct Handler<M> where M:Send {
    tx: mpsc::Sender<M>,
    // room_sender: Sender;
}
impl<M> Handler<M> where M:Send+'static {
    fn new(tx:mpsc::Sender<M>) -> Self {
        Self {
            tx: tx,
        }
    }
}

impl<M> ws::Handler for Handler<M> where M:Send+'static {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {

        println!("on request {:?}", thread::current().id());
        let mut res = try!(ws::Response::from_request(req));
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)

    }

    // Pass through any other methods that should be delegated to the child.
    //
    // You could probably use a macro for this if you have many different
    // routers or were building some sort of routing framework.

    fn on_shutdown(&mut self) {
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // self.room_sender.send(msg);
        //login request?
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
    }

    fn on_error(&mut self, err: ws::Error) {
    }
}


#[derive(Default)]
struct Lobby<M> where M:Send+'static {
    receivers: Vec<(mpsc::Receiver<M>,ws::Sender)>,
}
impl<M> Lobby<M> where M:Send+Default+'static {
    fn new() -> Lobby<M> {
        Lobby::<M> {
            ..Default::default()
//            receivers: Vec::new(),
        }
    }
    fn poll<F>(&mut self, mut handler:F) where F:FnMut(&M) -> bool {
        self.receivers.retain(|entry| {
            if let Ok(cmd) = entry.0.try_recv() {
                if handler(&cmd) {
                    return false;
                }
            }
            true
        });
    }
    fn accept(&mut self, rx:mpsc::Receiver<M>, sender:ws::Sender) {
        self.receivers.push((rx,sender));
    }
}

pub fn sample1_start() {
    let room_tx = room::start_thread::<game::Sample1Room>();
    let lobby = Arc::new(Mutex::new(Lobby::<String>::new()));
    
    let lobby_clone = lobby.clone();
    thread::spawn(move||{
        if let Err(error) = ws::listen("127.0.0.1:3012", |out| {
//            println!("connect {:?}", thread::current().id());
            let (tx,rx) = mpsc::channel();
            let mut lb = lobby_clone.lock().unwrap();
            lb.accept(rx, out);
            Handler{tx}
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });

    loop {
        let mut lb = lobby.lock().unwrap();
        lb.poll(|msg|{
            if msg == "login" {
                return true
            }
            false
        });
        thread::sleep(Duration::from_millis(1000));
    }

}

fn main() {
    sample1_start();
}
