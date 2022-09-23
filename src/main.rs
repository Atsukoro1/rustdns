#[macro_use] 
extern crate enum_primitive;
extern crate slog_async;
extern crate slog_term;
extern crate slog;
extern crate bit;

use crate::helpers::config::Config;
use crate::parser::dns::DNS;
use lazy_static::lazy_static;
use crate::cache::def::{
    CacheManager, CMTrait
};
use tokio::sync::{
    MutexGuard, 
    Mutex
};
use std::io::Write;
use std::{thread, io};
use std::net::{
    UdpSocket, 
    SocketAddr
};
use std::time::Duration;
use slog::{
    o, 
    Drain, 
    info, 
    crit, warn
};

mod parser;
mod helpers;
mod cache;

lazy_static! {
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
    
        slog::Logger::root(drain, o!("RUSTDNS" => "Rust dns is starting!"))
    };

    pub static ref CONFIG: Config = {
        let cfg: Config = match helpers::config::load_config() {
            Ok(conf) => {
                info!(LOGGER, "Succefully loaded config!");
                conf
            },

            Err(err) => {
                warn!(
                    LOGGER, 
                    "Something happened while loading config!";
                    "Details" => format!("{:?}", err)
                );

                // This is done to prevent panic printing above the logger message
                std::thread::sleep(Duration::from_millis(100));

                panic!()
            }
        };

        cfg
    };

    pub static ref SOCKET: UdpSocket = {
        let sckt = match UdpSocket::bind(
            format!("{}:{}", CONFIG.host.hostname, CONFIG.host.port)
                .as_str()
                .parse::<SocketAddr>()
                .unwrap()
        ) {
            Ok(s) => {
                info!(
                    LOGGER, 
                    "UDP socket is running!";
                    "host" => format!("{}:{}", CONFIG.host.hostname, CONFIG.host.port)
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
    println!("{:?}", DNS::from(&*bytes).unwrap());
    
    bytes.into_iter()
        .for_each(|bits| {
            print!("{:#010b}", bits);
            print!(", ");
        });

    println!("");
    println!("");

    DNS::from(&*bytes).unwrap()
        .bytes()
        .unwrap()
        .into_iter()
        .for_each(|bits| {
            print!("{:#010b}", bits);
            io::stdout().flush().unwrap();
            print!(", ");
            io::stdout().flush().unwrap();
        });
}

#[tokio::main]
async fn main() {
    let mut current_cm: MutexGuard<CacheManager> = CACHEMANAGER.lock()
        .await;

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