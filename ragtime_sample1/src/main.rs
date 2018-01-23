#[macro_use]
extern crate ragtime;
extern crate ws;

struct Router {
    // sender: ws::Sender,
    // inner: Box<ws::Handler>,
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {

        println!("====== {}", req.resource());
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
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
    }

    fn on_error(&mut self, err: ws::Error) {
    }
}

struct Sample1Game {
//    game_room_mgr:RoomManager<Sample1Room>,
}

impl Sample1Game {
    fn new() -> Sample1Game {
        Sample1Game{}
    }
    fn update(&mut self) {
    }
    fn on_connect(&self, sender: ws::Sender) -> Router {
        Router{}
    }
}

pub fn sample1_start() {
    let mut game = Sample1Game::new();
    // loop {
    //     game.update();
    //     thread::sleep(Duration::from_millis(1000));
    // }
   if let Err(error) = ws::listen("127.0.0.1:3012", |out| {
       game.on_connect(out)
    })
     {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

fn main() {
    sample1_start();
}
