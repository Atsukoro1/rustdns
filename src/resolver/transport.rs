use crate::helpers::bit::prepend;
#[allow(dead_code)]

use crate::{
    parser::dns::DNS,
    convert_u16_to_two_u8s
};
use async_recursion::async_recursion;
use std::{net::{
    UdpSocket, 
    SocketAddr, 
    Ipv4Addr, 
    IpAddr, TcpStream
}, io::{Write, Read}};

enum_from_primitive! {
    #[repr(u8)]
    #[derive(Debug)]
    pub enum TransportError {
        ClientInstantiateError = 0x0,
        ReadError = 0x2,
        WriteError = 0x3
    }
}

#[derive(PartialEq, Debug)]
pub enum TransportProto {
    TCP,
    UDP
}

/// This helper transport function is used to send payload with either TCP or UDP
/// client and receive payload back one time.
///
/// UDP will be used first, if the DNS message is truncated, then TCP will be used.
/// That means response latency will be significantly longer if TCP is required to be used.
///
/// Returns DNS struct on success and Transport on error
#[async_recursion]
pub async fn onetime_transport(payload: &[u8], host: SocketAddr, proto: Option<TransportProto>) -> Result<DNS, TransportError> {
    match proto {
        Some(TransportProto::UDP) | None => {
            /*
                We are creating the socket on port 0, which means OS will assign 
                available port on it's own
            */
            let mut socket: Result<std::net::UdpSocket, std::io::Error> = UdpSocket::bind(
                SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0
                )
            );

            if socket.is_err() {
                return Err::<DNS, TransportError>(
                    TransportError::ClientInstantiateError
                );
            };

            let transport: Result<usize, std::io::Error> = socket.as_mut().unwrap().send_to(
                payload, 
                host
            );

            if transport.is_err() {
                if proto == Some(TransportProto::UDP) {
                    return onetime_transport(
                        payload, 
                        host, 
                        Some(TransportProto::TCP)
                    ).await;
                }

                return Err::<DNS, TransportError>(
                    TransportError::WriteError
                );
            }

            let mut buf: [u8; 512] = [1; 512];
            socket.unwrap().peek(&mut buf)
                .expect("Failed");

            let datagram = DNS::from(&buf);

            /* 
                Check if message length is bigger than this actual datagram length.
                This is the whole purpose of this function.
            */
            if datagram.unwrap().header.truncated {
                return onetime_transport(
                    payload, 
                    host, 
                    Some(TransportProto::TCP)
                ).await;
            }
        },

        Some(TransportProto::TCP) => {
            let mut stream: std::io::Result<TcpStream> = TcpStream::connect(host);

            if stream.is_err() {
                return Err::<DNS, TransportError>(
                    TransportError::ClientInstantiateError   
                );
            };

            let u8_len: [u8; 2] = convert_u16_to_two_u8s!(payload.len() as u16, u16);
            let vec_u8 = prepend(payload.to_vec(), &u8_len);
            let write_str = stream.as_mut()
                .unwrap()
                .write(
                    &*vec_u8
                );

            if write_str.is_err() {
                return Err::<DNS, TransportError>(
                    TransportError::WriteError
                );
            };

            let mut buf: [u8; 2048] = [1; 2048];
            let read_str = stream.unwrap().read(&mut buf);

            if read_str.is_err() {
                return Err::<DNS, TransportError>(
                    TransportError::ReadError
                );
            };

            let datagram = DNS::from(&buf);
        }
    }

    Err(TransportError::ClientInstantiateError)
}