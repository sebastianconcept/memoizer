use std::cell::{RefCell, RefMut};

pub(crate) use serde::{Deserialize, Serialize};
use serde_json::{self, Result, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{io::BufReader, net::TcpStream};

use crate::storage::{get, reset, set, size};

// A MemoizerMessage is used to receive a command (selector) and an argument (its payload)
// so the service can perform one of actions supported by its `route` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

pub fn respond(message: &str, stream: &mut RefMut<'_, TcpStream>) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

// Performs the corresponding action for the given
// MemoizerMessage and responds its answer using the given stream.
pub fn route(message: MemoizerMessage, mut stream: &mut RefMut<'_, TcpStream>) {
    match message.s.as_str() {
        "get" => {
            let key = message.p["k"].to_string();
            let value = get(key);
            match value {
                None => respond("null", &mut stream),
                Some(v) => respond(&v, &mut stream),
            }
        }
        "set" => {
            let key = message.p["k"].to_string();
            let value = message.p["v"].to_string();
            set(key, value);
            respond("ok", &mut stream)
        }
        "reset" => {
            reset();
            respond("ok", &mut stream)
        }
        "size" => {
            let size = size();
            respond(&format!("{}", size), &mut stream)
        }
        _ => {
            println!("Received and unsupported value");
            respond(&format!("nok: {:?}", message), &mut stream)
        }
    }
}

// Handler that responds to the MemoizerMessage on the given line and stream.
fn on_line_received(message: Result<MemoizerMessage>, mut stream: &mut RefMut<'_, TcpStream>) {
    match message {
        Ok(m) => route(m, &mut stream),
        Err(err) => {
            println!("Received and unsupported value");
            let error_message = format!("{:?}", err);
            respond(&error_message, &mut stream)
        }
    }
}

// Handler for a new incoming connection.
// Will parse content as one MemoizerMessage per line.
pub async fn on_socket_accept(stream: &RefCell<TcpStream>) -> std::io::Result<()> {
    let mut stream1 = stream.borrow_mut();
    let source = BufReader::new(&mut *stream1);
    let mut lines = source.lines();
    while let Some(line) = lines.next_line().await? {
        let message: Result<MemoizerMessage> = serde_json::from_str(&line);
        let mut stream2 = stream.borrow_mut();
        on_line_received(message, &mut stream2);
    }
    Ok(())
}