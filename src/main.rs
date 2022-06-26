use serde::{Deserialize, Serialize};
use serde_json::{self, Error, Result, Value};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::{fs, thread};

#[derive(Serialize, Deserialize)]
struct MemoizerMessage {
    s: String, // selector
    p: Value, // payload
}

pub static SOCKET_PATH: &'static str = "/tmp/memoizer-socket";

fn to_str(string: &str) -> &str {
    string
}

fn respond(message: &str, mut stream: &UnixStream) {
    let paylaod = format!("{}\n\r", message);
    stream.write_all(paylaod.as_bytes());
}

fn on_line_received(line: String, stream: &UnixStream) {
    println!("On line received: {:?}", line);
    let m: Result<MemoizerMessage> = serde_json::from_str(&line);
    match m {
        Ok(m) => {
            println!("Received a MemoizerMessage");
            respond("ok", stream)
        }
        Err(err) => {
            println!("Received and unsupported value");
            respond("nok", stream)
        } 
        // match value["m"] {
          //     "get" => {
          //         println!("Received a get");
          //         respond("ok", stream)
          //     },
          //     "set" => {
          //         println!("Received a get");
          //         respond("ok", stream)
          //     },
          //     _ => {
          //         println!("Received and unsupported value");
          //         respond("nok", stream)
          //     }
          // }
    }
}

fn value_from(data: &str) -> Result<Value> {
    serde_json::from_str(data)
}

// fn on_line_received(line: String, stream: &UnixStream) {
//     let value = value_from(&line);
//     match value {
//         Ok(value) => {
//             on_value_received(value, &stream);
//         }
//         Err(e) => println!("Invalid message: {:?}", e),
//     }
// }

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
