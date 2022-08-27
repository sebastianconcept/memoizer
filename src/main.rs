#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use tokio::net::TcpListener;

use crate::config::*;
use crate::listener::*;
use std::cell::RefCell;
use std::error::Error;

pub mod config;
mod listener;
pub mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut threads = vec![];
    
    // Bind listener to socket
    let socket_address = get_socket_address();
    let listener = TcpListener::bind(&socket_address).await?;
    println!("Accepting incoming connection: {:?}", &socket_address);
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        let thread = tokio::spawn(async move {
            let mut stream = RefCell::new(socket);
            on_socket_accept(&stream);
        });
        threads.push(thread);
    }
}
