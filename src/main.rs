#[macro_use] 
extern crate enum_primitive;
extern crate bit;

use std::net::SocketAddr;
use std::thread;
use std::net::UdpSocket;

mod parser;
mod helpers;

fn handle_datagram(bytes: &[u8], _src: SocketAddr) {
    let datagram: parser::def::DNS = <parser::def::DNS as parser::def::Construct>::from(bytes);
    println!("{:?}", datagram);
}

fn main() {
    let socket = match UdpSocket::bind("127.0.0.1:53") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    loop {
        let mut buf = [0; 520];

        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    handle_datagram(
                        &buf[0..amt],
                        src
                    );
                });
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}