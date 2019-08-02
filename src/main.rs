extern crate ws;

use std::env;
use std::error::Error;
use std::process;
use ws::{listen, Handler, Sender, Handshake, Message, Factory};

struct WsHandler {
    ws: Sender,
    isClient: bool
}

impl Handler for WsHandler {
    fn on_open(&mut self, handShake: Handshake) -> ws::Result<()>{
        if self.isClient {
            println!("Server have a connection");
        }
        else {
            println!("Client connected");
        }

        Ok(())
    }
}

struct Server{
}

impl Default for Server {
    fn default() -> Self {
        Server {}
    }
}

impl Factory for Server {
    type Handler = WsHandler;

    fn connection_made(&mut self, ws: Sender) -> WsHandler {
        WsHandler {
            ws: ws,
            isClient: false
        }
    }

    fn client_connected(&mut self, ws: Sender) -> WsHandler {
        println!("client connected");
        WsHandler {
            ws: ws,
            isClient: true
        }
    }
}

fn getAddr(mut args: std::env::Args) -> Result<String, &'static str>{
    args.next();
    let addr = match args.next() {
        Some (arg) => {
            arg
        },
        None => {
            return Err("Didn't get server address")
        }
    };

    Ok(addr)
}

fn main() {
    let addr = getAddr(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("address: {}", addr);

    let ws = ws::WebSocket::new(Server::default()).expect("cannot create server");
    ws.listen(addr).unwrap();

}