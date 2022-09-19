#[macro_use] 
extern crate enum_primitive;
extern crate bit;

use lazy_static::lazy_static;
use tokio::sync::{Mutex, MutexGuard};
use std::net::{UdpSocket, SocketAddr};
use std::thread;
use crate::helpers::config::Config;
use crate::cache::def::{CacheManager, CMTrait};

mod parser;
mod helpers;
mod cache;

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
            Ok(s) => {
                println!("UDP socket succefully started!");
                s
            },
            
            Err(e) => panic!("couldn't bind socket: {}", e)
        };

        sckt
    };

    pub static ref CACHEMANAGER: tokio::sync::Mutex<CacheManager> = Mutex::new({
        let mut manager = CacheManager::new();

        match manager.connect() {
            Ok(_) => println!("Cache manager started!"),
            Err(e) => panic!("Failed to start cache manager, {}", e)
        }

        manager
    });
}

fn handle_datagram(bytes: &[u8], _src: SocketAddr) {
    let datagram: parser::def::DNS = <parser::def::DNS as parser::def::Construct>::from(bytes)
        .unwrap();

    println!("{:?}", datagram);
}

#[tokio::main]
async fn main() {
    let mut current_cm: MutexGuard<CacheManager> = CACHEMANAGER.lock()
        .await;

    current_cm.connect()   
        .expect("Failed to connect to cache manager");

    current_cm.load_resources()
        .await
        .expect("Failed to load resources");

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