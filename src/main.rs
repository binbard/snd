use std::{io, net::Ipv4Addr};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let local_addr = (Ipv4Addr::UNSPECIFIED, 8051);
    let sock = UdpSocket::bind(local_addr).await?;
    println!("{:?}", sock.local_addr());

    loop {
        let mut buf = [0; 1];

        if let Ok((len, addr)) = sock.recv_from(&mut buf).await {
            if &buf[..len] == b"C" {
                println!("Got '{:?}' request from {:?}", &buf[..len], addr);
                let len = sock.send_to(&buf[..len], addr).await?;
                println!("Sent {:?} byte back", len);
            }
        }
    }
}
