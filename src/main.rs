use tokio::net::{UdpSocket};
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {

    let local_addr = ("0.0.0.0", 8051);

    let usock = UdpSocket::bind(local_addr).await?;
    println!("UDP Listening on {:?}", usock.local_addr());

    loop {
        let mut buf = [0; 1];

        if let Ok((len, addr)) = usock.recv_from(&mut buf).await {
            if &buf[..len] == b"C" {
                println!("Got '{:?}' from {:?}", &buf[..len], addr);
                let len = usock.send_to(&buf[..len], addr).await?;
                println!("{:?} bytes sent", len);
            }
        }
        
    }
}