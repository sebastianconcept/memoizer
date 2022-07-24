pub(crate) use serde::{Deserialize, Serialize};
use serde_json::{self, Result, Value};

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::storage::{get, reset, set, size};

// A MemoizerMessage is used to receive a command (selector) and an argument (its payload)
// so the service can perform one of actions supported by its `route` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

pub fn respond(message: &str, mut stream: &TcpStream) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

// Performs the corresponding action for the given 
// MemoizerMessage and responds its answer using the given stream.
pub fn route(message: MemoizerMessage, mut stream: &TcpStream) {
    match message.s.as_str() {
        "get" => {
            let key = message.p["k"].to_string();
            let value = get(key);
            match value {
                None => respond("null", stream),
                Some(v) => respond(&v, stream),
            }
        }
        "set" => {
            let key = message.p["k"].to_string();
            let value = message.p["v"].to_string();
            set(key, value);
            respond("ok", stream)
        }
        "reset" => {
            reset();
            respond("ok", stream)
        }
        "size" => {
            let size = size();
            respond(&format!("{}", size), stream)
        }
        _ => {
            println!("Received and unsupported value");
            respond(&format!("nok: {:?}", message), stream)
        }
    }
}

// Handler that responds to the MemoizerMessage on the given line and stream.
fn on_line_received(line: String, stream: &TcpStream) {
    let m: Result<MemoizerMessage> = serde_json::from_str(&line);
    match m {
        Ok(m) => route(m, stream),
        Err(err) => {
            println!("Received and unsupported value");
            let error_message = format!("{:?}", err);
            respond(&error_message, stream)
        }
    }
}

// Handler for a new incoming connection.
// Will parse content as one MemoizerMessage per line.
pub fn on_socket_accept(stream: &TcpStream) {
    println!("Accepting incoming connection: {:?}", stream.local_addr());
    let source = BufReader::new(stream);
    for line in source.lines() {
        on_line_received(line.unwrap(), stream);
    }
}
