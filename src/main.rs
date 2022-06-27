#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use std::{fs, os::unix::net::UnixListener, path::Path, thread};

use crate::listener::*;

mod listener;
pub mod storage;

fn main() {
    let socket = Path::new(SOCKET_PATH);

    // Delete old socket if necessary
    if socket.exists() {
        fs::remove_file(&socket).ok();
    }

    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("Failed to bind socket"),
        Ok(listener) => listener,
    };

    println!("Server started, waiting for clients");

    listener.incoming().for_each(|stream| match stream {
        Ok(stream) => {
            thread::spawn(move || on_socket_accept(&stream));
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    });
}
