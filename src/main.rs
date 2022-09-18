#[macro_use] 
extern crate enum_primitive;
extern crate bit;

use parser::parse::datagram_bytes;
use lazy_static::lazy_static;
use std::net::UdpSocket;
use redis::Connection;
use parser::def::{
    ErrorCode, 
    OpCode, 
    DNSResourceFormat, 
    Type
};
use std::thread;
use std::net::{
    SocketAddr
};

use crate::helpers::config::Config;

mod parser;
mod helpers;

lazy_static! {
    pub static ref CONFIG: Config = {
        helpers::config::load_config()
    };

    pub static ref SOCKET: UdpSocket = {
        let sckt = match UdpSocket::bind(
            format!("{}:{}", CONFIG.hostname, CONFIG.port)
                .as_str()
                .parse::<SocketAddr>()
                .unwrap()
        ) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e)
        };

        sckt
    };

    pub static ref REDIS: Connection = {
        redis::Client::open(&*CONFIG.redis_addr)
            .expect("Failed to estabilish connection with redis cache")
            .get_connection()
            .expect("Failed to get redis connection")
    };
}

fn handle_datagram(bytes: &[u8], src: SocketAddr) {
    let mut datagram: parser::def::DNS = <parser::def::DNS as parser::def::Construct>::from(bytes)
        .unwrap();

    if datagram.questions[0].name == "hoohle.com" {
        datagram.header.qr = Type::Response;
        datagram.header.error_code = ErrorCode::FormatError;
        datagram.header.op_code = OpCode::Query;
    
        SOCKET.send_to(&datagram_bytes(datagram), src).expect("Failed");
    } else {
        datagram.header.qr = Type::Response;
        datagram.header.op_code = OpCode::Query;
        datagram.header.answer_count = 1;
        datagram.answer = Some(DNSResourceFormat {
            name: String::from("google.com"),
            rr_type: parser::def::QuestionType::A,
            rr_class: parser::def::QuestionClass::IN,
            data: String::from("124.34.2.1"),
            ttl: 100000,
            length: 4
        });

        SOCKET.send_to(&datagram_bytes(datagram), src).expect("Failed");
    }

}

fn main() {
    loop {
        let mut buf = [0; 512];

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