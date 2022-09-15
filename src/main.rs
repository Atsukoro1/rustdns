#[macro_use] 
extern crate enum_primitive;
extern crate bit;

use std::thread;
use std::net::UdpSocket;
use std::net::{
    SocketAddr
};

mod parser;
mod helpers;

fn handle_datagram(bytes: &[u8], _src: SocketAddr) {
    let datagram: parser::def::DNS = <parser::def::DNS as parser::def::Construct>::from(bytes);
    println!("{:?}", datagram);
}

fn main() {
    let config = helpers::config::load_config();

    let socket = match UdpSocket::bind(
        format!("{}:{}", config.hostname, config.port)
            .as_str()
            .parse::<SocketAddr>()
            .unwrap()
    ) {
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