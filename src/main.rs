use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {

    let socket = match UdpSocket::bind("192.168.0.15:22") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    let mut buf = [0; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                });
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}