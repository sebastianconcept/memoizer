#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use tokio::net::TcpListener;

use crate::config::*;
use crate::listener::*;
use std::error::Error;

pub mod config;
mod listener;
pub mod storage;

use storage::benchmark;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (times, payload) = get_bench_and_payload();
    if let Some(t) = times {
        benchmark(t, payload);
    }

    let socket_address = get_socket_address();
    let listener = TcpListener::bind(&socket_address).await?;
    println!("Memoizer listening on {}", socket_address);
    loop {
        let (socket, _) = listener.accept().await?;
        let _thread = tokio::spawn(async move {
            on_socket_accept(socket)
                .await
                .expect("Failed to process incoming socket connection")
        });
    }
}
