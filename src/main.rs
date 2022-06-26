// #![feature(io)]
// #![feature(path)]

use std::io::Read;
// use std::net::SocketAddr;
use std::os::unix::net::{UnixListener, UnixStream, SocketAddr};
use std::path::Path;
use std::{thread, fs};

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn on_socket_accept(mut stream: UnixStream, addr: SocketAddr){
    println!("Accepting incoming connection: {:?}", addr);
    let mut message = String::new();
    stream.read_to_string(&mut message);
    println!("Client said: {}", message);
}

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

    match listener.accept() {
        Ok((mut stream, addr)) => {
            on_socket_accept(stream, addr)
        }
        Err(e) => println!("accept function failed: {e:?}"),
    }
}
