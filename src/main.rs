use std::net::SocketAddr;
use std::thread;
use std::net::UdpSocket;

mod parse;
mod helper;

fn handle_datagram(bytes: &[u8], _src: SocketAddr) {
    let header: parse::DNSHeader = parse::parse_datagram(bytes);
    println!("{:?}", header);
}

fn main() {
    let socket = match UdpSocket::bind("192.168.0.15:53") {
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