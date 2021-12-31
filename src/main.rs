// jkcoxson

use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self, target: String) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        let mut original_gangster = "".to_string();
        let target_gangster = format!("{}:24642", target);

        loop {
            if let Some((size, peer)) = to_send {
                println!("Peer: {}", peer);
                if peer.to_string() == target_gangster {
                    let amt = socket.send_to(&buf[..size], &original_gangster).await?;
                    println!("Sent {} bytes to {}", amt, &original_gangster)
                } else {
                    if original_gangster == "".to_string() {
                        original_gangster = peer.to_string()
                    }
                    let amt = socket.send_to(&buf[..size], &target_gangster).await?;
                    println!("Sent {} bytes to {}", amt, &target_gangster)
                }
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Enter a target address");
        panic!();
    }

    let socket = UdpSocket::bind("0.0.0.0:24642").await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 10240],
        to_send: None,
    };

    // This starts the server task.
    server.run(args[1].clone()).await?;

    Ok(())
}
