extern crate ws;

use std::env;
use std::error::Error;
use std::process;
use ws::{listen, Handler, Sender, Handshake, Message};

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

    listen(addr, |out| {
        move |msg| {
            out.send(msg)
        }
    }).unwrap();
}