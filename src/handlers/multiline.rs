use std::collections::HashSet;
use std::fmt::format;
use std::io::stdin;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

pub fn multiline_listen(socket: &UdpSocket, myname: String) {
    let multicast_group = Ipv4Addr::new(224, 0, 0, 1);

    socket
        .join_multicast_v4(&multicast_group, &Ipv4Addr::UNSPECIFIED)
        .unwrap();

    let socket_clone = socket.try_clone().unwrap();

    let th = thread::spawn(move || {
        let mut msg = [0u8; 1024];

        loop {
            match socket_clone.recv_from(&mut msg) {
                Ok((amt, src)) => {
                    let code = msg[0];
                    let msg = String::from_utf8_lossy(&msg[1..amt]).to_string();
                    match code {
                        1 => {
                            println!("1 Connection Request from {}", src);
                        }
                        4 => {
                            println!("4 Broadcast Message from {}: {}", src, msg);
                        }
                        6 => {
                            println!("6 Message from {}: {}", src, msg);
                        }
                        9 => {
                            println!("9 Username query from {}: {}", src, msg);
                            if(msg == myname) {
                                print!("{} is me", msg);
                                let res = format!("{}{}", 10 as char, myname);
                                socket_clone.send_to(res.as_bytes(), src).unwrap();
                            }
                        }
                        _ => println!("_Unknown Message from {}: {}{}", src, code, msg),
                    }
                }
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::ConnectionReset {
                        println!("Connection reset by peer");
                    } else {
                        panic!("Error receiving data: {}", err);
                    }
                }
            }
        }
    });
    // th.join().unwrap();
}

pub fn local_room_send(code: u8, msg: String, socket: &UdpSocket) {
    let msg = format!("{}{}", code as char, msg);
    socket.send_to(msg.as_bytes(), "224.0.0.1:4000").unwrap();
    // println!("Sent: {}", input);
}

pub fn local_room_chat(socket: &UdpSocket, myname: String) -> thread::JoinHandle<()> {
    let socket_clone = socket.try_clone().expect("Failed to clone socket");

    multiline_listen(&socket, myname);

    let th = thread::spawn(move || loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let msg = input.pop();
        local_room_send(4, input, &socket_clone);
    });
    th
}

pub fn local_query(code: u8, query: String, ack: bool) -> String {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:4002").unwrap();
    let socket_clone: UdpSocket = socket.try_clone().expect("Failed to clone socket");

    socket
        .join_multicast_v4(&Ipv4Addr::new(224,0,0,1), &Ipv4Addr::UNSPECIFIED)
        .unwrap();

    let th = thread::spawn(move || {
        let msg = format!("{}{}", code as char, query);
        for _ in 0..3 {
            socket_clone
                .send_to(msg.as_bytes(), "224.0.0.1:4000")
                .unwrap();
        }
    });

    if (ack == false) {
        return "".to_string();
    }

    let mut sources: HashSet<String> = HashSet::new();
    let mut result = "".to_string();


    for _ in 0..1{
        let mut msg = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut msg).unwrap();
        if (sources.contains(&src.to_string())) {
            println!("Duplicate from {}", src);
        }
        let code = msg[0];
        let msg: String = String::from_utf8_lossy(&msg[1..amt]).to_string();
        match code {
            10 => {
                println!("ZUsername ACK from {}", src.ip().to_string());
                println!("RES_{result}");
                result = format!("{}{}|{}{}", result, src.ip().to_string(), msg, 0 as char);
                sources.insert(src.to_string());
            }
            _ => println!("ZUnknown Message from {}: {}{}", src, code, msg),
        }
    }

    // println!("DONEEEE");

    th.join().unwrap();

    result.trim_end().to_string()
}
