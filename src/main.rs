use std::net::{TcpListener, TcpStream};
use std::thread;
use connection::Player;

// traits
use std::io::Read;
use std::io::Write;

mod connection;
mod commands;
mod logon;
mod storage;
mod socketutilities;
mod character;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    let mut player = Player::new(stream);
					player.handle_client();
                });
            }
        }
    }
}