use crate::helpers::bit::prepend;
#[allow(dead_code)]

use crate::{
    parser::dns::DNS,
    convert_u16_to_two_u8s,
    convert_two_u8s_to_u16
};
use async_recursion::async_recursion;
use std::io::{Read, Write};
use std::{net::{
    UdpSocket, 
    SocketAddr, 
    Ipv4Addr, 
    IpAddr, TcpStream
}};

enum_from_primitive! {
    #[repr(u8)]
    #[derive(Debug)]
    pub enum TransportError {
        /*
            Client cannot be instantiated probably because ip or port is 
            already taken or this process does not have permissions to 
            use the port
        */
        ClientInstantiateError = 0x0,

        // Something happened when reading the received buffer
        ReadError = 0x2,

        // Something happened when writing the payload
        WriteError = 0x3,

        // Datagram length is smaller than it should be
        DatagramLengthError = 0x4
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

            let datagram = DNS::from(&buf, TransportProto::UDP);

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
            let read_str: Result<usize, std::io::Error> = stream.unwrap().read(&mut buf);

            if read_str.is_err() {
                return Err::<DNS, TransportError>(
                    TransportError::ReadError
                );
            };

            // Remove unused bytes from the byte slice
            let sliced: &[u8] = &buf[0..read_str.unwrap()];

            /*
                Since datagram can't be parsed, because we don't know if we
                have the whole datagram, length will be extracted from the 
                the first two bytes of the buffer
            */
            let buf_len = convert_two_u8s_to_u16!(sliced[0], sliced[1]);

            // Check if datagram arrived in correct length
            if buf_len != (sliced.as_ref().len() as u16 - 2) {
                return Err::<DNS, TransportError>(
                    TransportError::DatagramLengthError
                );
            };

            let datagram = DNS::from(sliced, TransportProto::TCP);
            
            return Ok(datagram.unwrap());
        }
    }

    Err(TransportError::ClientInstantiateError)
}