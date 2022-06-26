use serde::{Deserialize, Serialize};
use serde_json::{self, Error, Result, Value};

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::{fmt, fs, thread};

#[derive(Serialize, Deserialize)]
struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn to_str(string: &str) -> &str {
    string
}

fn respond(message: &str, mut stream: &UnixStream) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

fn route(message: MemoizerMessage, mut stream: &UnixStream) {
    if "get".to_string() == message.s {
        println!("Received a get");
       return respond("ok", stream)
    }
    if "set".to_string() == message.s  {
        println!("Received a set");
        return respond("ok", stream)
    }

    println!("Received and unsupported value");
    respond("nok", stream)
}

fn on_line_received(line: String, stream: &UnixStream) {
    println!("On line received: {:?}", line);
    let m: Result<MemoizerMessage> = serde_json::from_str(&line);
    match m {
        Ok(m) => {
            println!("Received a MemoizerMessage");
            route(m, stream)
        }
        Err(err) => {
            println!("Received and unsupported value");
            let error_message = format!("{:?}", err);
            respond(&error_message, stream)
        }
    }
}

fn on_socket_accept(stream: &UnixStream) {
    println!("Accepting incoming connection: {:?}", stream.local_addr());
    let receiver = BufReader::new(stream);
    for line in receiver.lines() {
        on_line_received(line.unwrap(), stream);
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
                thread::spawn(move || on_socket_accept(&stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
