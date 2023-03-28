use tokio::net::UdpSocket;
use std::io;
use tokio::time::{Duration, interval};

async fn send_hi_to_lan(sock: &UdpSocket, broadcast_addr: &str) -> io::Result<()> {
    let message = "C";
    sock.send_to(message.as_bytes(), broadcast_addr).await?;
    println!("Sent '{}' to all devices on the LAN", message);
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8050").await?;
    let broadcast_addr = "192.168.1.25:8051";
    let mut buf = [0; 1];

    let mut interval = interval(Duration::from_secs(10));
    let sock_addr = sock.local_addr()?;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                send_hi_to_lan(&sock, &broadcast_addr).await?;
            }
            result = sock.recv_from(&mut buf) => {
                let (len, addr) = result?;
                if addr == sock_addr {
                    continue;
                }
                let message = std::str::from_utf8(&buf[..len]).unwrap();
                println!("Received message '{}' from {:?}", message, addr);
                let len = sock.send_to(&buf[..len], addr).await?;
                println!("{:?} bytes sent", len);
            }
        }
    }
}

