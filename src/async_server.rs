use runtime::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::error::*;
use crate::packet::*;
use crate::read_req::*;

pub struct AsyncTftpServer {
    socket: UdpSocket,
}

impl AsyncTftpServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(AsyncTftpServer {
            socket: UdpSocket::bind(addr)?,
        })
    }

    pub fn with_socket(socket: UdpSocket) -> Result<Self> {
        Ok(AsyncTftpServer {
            socket,
        })
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket.local_addr()?)
    }

    pub async fn serve(&mut self) -> Result<()> {
        let mut buf = vec![0u8; 1024];

        loop {
            let (len, peer) = self.socket.recv_from(&mut buf).await?;

            let packet = match Packet::from_bytes(&buf[..len]) {
                Ok(x) => x,
                Err(_) => continue,
            };

            match packet {
                Packet::Rrq(req) => {
                    runtime::spawn(handle_rrq(req, peer));
                }
                Packet::Wrq(_req) => {
                    // TODO
                }
                // ignore packets that are not requests
                _ => continue,
            }
        }
    }
}

async fn handle_rrq(req: RwReq, peer: SocketAddr) {
    let mut read_req = match ReadRequest::init(peer, req) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to bind udp socket: {:?}", e);
            return;
        }
    };
    read_req.handle().await;
}
