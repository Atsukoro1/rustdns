#[macro_use] 
extern crate enum_primitive;
extern crate slog_async;
extern crate slog_term;
extern crate slog;
extern crate bit;

use lazy_static::lazy_static;
use slog::{o, Drain, info, crit};
use tokio::sync::{MutexGuard, Mutex};
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

    pub static ref LOGGER: slog::Logger = {
        let decorator = slog_term::TermDecorator::new()
            .stdout()
            .force_color()
            .build();

        let drain = slog_term::CompactFormat::new(decorator)
            .build()
            .fuse();

        let drain = slog_async::Async::new(drain)
            .build()
            .fuse();
    
        slog::Logger::root(drain, o!("RUSTDNS" => "Rust dns is running!"))
    };

    pub static ref SOCKET: UdpSocket = {
        let sckt = match UdpSocket::bind(
            format!("{}:{}", CONFIG.hostname, CONFIG.port)
                .as_str()
                .parse::<SocketAddr>()
                .unwrap()
        ) {
            Ok(s) => {
                info!(
                    LOGGER, 
                    "UDP socket is running!";
                    "host" => format!("{}:{}", CONFIG.hostname, CONFIG.port)
                );
                s
            },
            
            Err(e) => panic!("couldn't bind socket: {}", e)
        };

        sckt
    };

    pub static ref CACHEMANAGER: tokio::sync::Mutex<CacheManager> = Mutex::new({
        let mut manager = CacheManager::new();

        match manager.connect() {
            Ok(_) => info!(LOGGER, "Cache manager succefully started!"),
            Err(e) => {
                crit!(LOGGER, "Failed to start cache manager!"; "Error" => e);
                panic!()
            }
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