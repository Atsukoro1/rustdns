#[macro_use] 
extern crate enum_primitive;
extern crate bit;

use crate::parser::def::Construct;
use lazy_static::lazy_static;
use std::net::UdpSocket;
use std::thread;
use std::net::{
    SocketAddr
};

mod parser;
mod helpers;

lazy_static! {
    static ref SOCKET: UdpSocket = {
        let config = helpers::config::load_config();

        let sckt = match UdpSocket::bind(
            format!("{}:{}", config.hostname, config.port)
                .as_str()
                .parse::<SocketAddr>()
                .unwrap()
        ) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e)
        };

        sckt
    };
}

fn handle_datagram(bytes: &[u8], _src: SocketAddr) {
    let datagram: parser::def::DNS = <parser::def::DNS as parser::def::Construct>::from(bytes);
    println!("{:?}", datagram.bytes());
}

fn main() {
    loop {
        let mut buf = [0; 520];

        match SOCKET.recv_from(&mut buf) {
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