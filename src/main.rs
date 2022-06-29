#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use crate::listener::*;
use std::net::TcpListener;
use std::thread;

mod listener;
pub mod storage;

fn main() {
    let socket_address = "127.0.0.1:9001";

    // Bind to socket
    let listener = match TcpListener::bind(&socket_address) {
        Err(_) => panic!("Failed to bind socket"),
        Ok(listener) => listener,
    };

    println!("Server started, waiting for clients on {}", socket_address);

    listener.incoming().for_each(|stream| match stream {
        Ok(stream) => {
            thread::spawn(move || on_socket_accept(&stream));
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    });
}
