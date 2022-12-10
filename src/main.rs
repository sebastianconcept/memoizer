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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket_address = get_socket_address();
    let listener = TcpListener::bind(&socket_address).await?;
    println!("Memoizer starting... {}", socket_address);
    loop {
        let (socket, _) = listener.accept().await?;
        let _thread = tokio::spawn(async move {
            on_socket_accept(socket)
                .await
                .expect("Failed to process incoming socket connection");
        });
    }
}
