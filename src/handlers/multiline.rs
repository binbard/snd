use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;

pub fn multiline_listen(socket: &UdpSocket) -> thread::JoinHandle<()> {
    let multicast_group = Ipv4Addr::new(224, 0, 0, 1);

    socket
        .join_multicast_v4(&multicast_group, &Ipv4Addr::UNSPECIFIED)
        .unwrap();

    let socket_clone = socket.try_clone().expect("Failed to clone socket");

    let th = thread::spawn(move || {
        let mut msg = [0u8; 1024];

        loop {
            let (amt, src) = socket_clone.recv_from(&mut msg).unwrap();
            let code = msg[0];
            let msg: String = String::from_utf8_lossy(&msg[1..amt]).to_string();
            match code {
                1 => {
                    println!("Connection Request from {}", src);
                }
                4 => {
                    println!("Broadcast Message from {}: {}", src, msg);
                }
                _ => println!("Unknown Message from {}: {}{}", src, code, msg),
            }
        }
    });
    th
}

pub fn multiline_send(code: u8, msg: String, socket: &UdpSocket) {
    let msg = format!("{}{}", code as char, msg);
    socket.send_to(msg.as_bytes(), "224.0.0.1:4000").unwrap();
    // println!("Sent: {}", input);
}
