use anyhow::Result;
pub(crate) use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{io::BufReader, net::TcpStream};

use crate::storage::{get, reset, set, size};

// A MemoizerMessage is used to receive a command (selector) and an argument (its payload)
// so the service can perform one of actions supported by its `route` method.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MemoizerMessage {
    s: String, // selector
    p: Value,  // payload
}

#[derive(Debug, Error)]
enum MessageProcessingError {
    #[error("message could not be parsed")]
    ParsingError,
    #[error("message not supported")]
    UnsupportedMessage,
}

// Performs the corresponding action for the given
// MemoizerMessage and returns the corresponding answer
pub fn route(message: &MemoizerMessage) -> Result<String> {
    match message.s.as_str() {
        "get" => {
            let key = message.p["k"].to_string();
            let value = get(key);
            match value {
                None => Ok("null".to_string()),
                Some(v) => Ok(v),
            }
        }
        "set" => {
            let key = message.p["k"].to_string();
            let value = message.p["v"].to_string();
            set(key, value);
            Ok("ok".into())
        }
        "reset" => {
            reset();
            Ok("ok".to_string())
        }
        "size" => {
            let size = size();
            Ok(format!("{}", size))
        }
        _ => {
            println!("Received an unsupported value {:?}", message);
            Err(MessageProcessingError::UnsupportedMessage.into())
        }
    }
}

// Handler that responds to the MemoizerMessage on the given line and stream.
async fn on_line_received(message: &serde_json::Result<MemoizerMessage>) -> Result<String> {
    match message {
        Ok(m) => route(m),
        Err(_) => Err(MessageProcessingError::ParsingError.into()),
    }
}

// Handler for a new incoming connection.
// Will parse content as one MemoizerMessage per line.
pub async fn on_socket_accept(mut stream: TcpStream) -> Result<String> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();
    while let Some(line) = lines.next_line().await? {
        let message = serde_json::from_str::<MemoizerMessage>(&line);
        let response = on_line_received(&message).await?;
        let paylaod = format!("{}\n\r", response);
        writer.write_all(paylaod.as_bytes()).await?
    }
    Ok("ok".to_string())
}
