pub(crate) use serde::{Deserialize, Serialize};
use serde_json::{self, Result, Value};

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::storage::{get, reset, set, size};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn to_str(string: &str) -> &str {
    string
}

pub fn respond(message: &str, mut stream: &TcpStream) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

pub fn route(message: MemoizerMessage, mut stream: &TcpStream) {
    match message.s.as_str() {
        "get" => {
            let key = message.p["k"].to_string();
            // println!("Received a get for k {}", key);
            let value = get(key);
            match value {
                None => respond("null", stream),
                Some(v) => respond(&v, stream),
            }
        }
        "set" => {
            let key = message.p["k"].to_string();
            let value = message.p["v"].to_string();
            // println!("Received a set for: \nk: {} \nv: {}", key, value);
            // println!("k: {}", key);
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

pub fn on_socket_accept(stream: &TcpStream) {
    println!("Accepting incoming connection: {:?}", stream.local_addr());
    // let protected_stream = MutStatic::from(stream);
    // let source: mut_static::ForceSomeRwLockReadGuard<&TcpStream> =
    //     protected_stream.read().ok().unwrap();
    let source = BufReader::new(stream);
    for line in source.lines() {
        on_line_received(line.unwrap(), stream);
    }
}
