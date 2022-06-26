use serde::{Deserialize, Serialize};
use serde_json::{self, Result, Value};

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

#[derive(Serialize, Deserialize)]
pub struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn to_str(string: &str) -> &str {
    string
}

pub fn respond(message: &str, mut stream: &UnixStream) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

pub fn route(message: MemoizerMessage, mut stream: &UnixStream) {
    match message.s.as_str() {
        "get" => {
            println!("Received a get");
            respond("ok", stream)
        }
        "set" => {
            println!("Received a set: {}", message.p);
            respond("ok", stream)
        }
        "reset" => {
            println!("Received a reset");
            respond("ok", stream)
        }
        _ => {
            println!("Received and unsupported value");
            respond("nok", stream)
        }
    }
}

fn on_line_received(line: String, stream: &UnixStream) {
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

pub fn on_socket_accept(stream: &UnixStream) {
    println!("Accepting incoming connection: {:?}", stream.local_addr());
    let receiver = BufReader::new(stream);
    for line in receiver.lines() {
        on_line_received(line.unwrap(), stream);
    }
}
