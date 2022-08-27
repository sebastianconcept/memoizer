

use std::error::Error;

pub(crate) use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
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

// Performs the corresponding action for the given
// MemoizerMessage and returns the corresponding answer
pub fn route(message: MemoizerMessage) -> String {
    match message.s.as_str() {
        "get" => {
            let key = message.p["k"].to_string();
            let value = get(key);
            match value {
                None => "null".to_string(),
                Some(v) => v,
            }
        }
        "set" => {
            let key = message.p["k"].to_string();
            let value = message.p["v"].to_string();
            set(key, value);
            "ok".to_string()
        }
        "reset" => {
            reset();
            "ok".to_string()
        }
        "size" => {
            let size = size();
            format!("{}", size)
        }
        _ => {
            println!("Received and unsupported value");
            format!("nok: {:?}", message)
        }
    }
}

// Handler that responds to the MemoizerMessage on the given line and stream.
fn on_line_received(message: serde_json::Result<MemoizerMessage>) -> String {
    match message {
        Ok(m) => route(m),
        Err(err) => {
            println!("Received and unsupported value");
            let error_message = format!("{:?}", err);
            error_message
        }
    }
}

// Handler for a new incoming connection.
// Will parse content as one MemoizerMessage per line.
pub async fn on_socket_accept(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();
    while let Some(line) = lines.next_line().await? {
        let message: serde_json::Result<MemoizerMessage> = serde_json::from_str(&line);
        let response = on_line_received(message);
        let paylaod = format!("{}\n\r", response);
        writer.write_all(paylaod.as_bytes())
            .await
            .expect("Failed to write to the socket");
    };
    Ok(())
}
