// #![feature(io)]
// #![feature(path)]

use serde_json::{Result, Value};
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::{fs, thread};

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn value_from(data: &str) -> Result<Value> {
    serde_json::from_str(data)
}

fn on_line_received(line: String) {
    // println!("data: {:?}", line);
    // let value = value_from(&line).unwrap();
    let value = value_from(&line).unwrap_or(value_from("{}").unwrap());
    println!("{}", value);
}

fn on_socket_accept(mut stream: UnixStream) {
    println!("Accepting incoming connection: {:?}", stream.local_addr());
    let receiver = BufReader::new(stream);
    for line in receiver.lines() {
        on_line_received(line.unwrap());
    }
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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| on_socket_accept(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
