use std::net::{Ipv4Addr, UdpSocket};

fn main(){

    let socket = UdpSocket::bind("0.0.0.0:4001").unwrap();

    let multicast_ip: Ipv4Addr = "224.0.0.1".parse().unwrap();
    socket.join_multicast_v4(&multicast_ip, &Ipv4Addr::from([4,0,0,0])).unwrap();

    let mut buf = [0u8; 1024];

    loop{
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("Received {} bytes from {}", amt, src);
        println!("buf: {:?}", std::str::from_utf8(&buf[..amt]).unwrap());
    }
    
}