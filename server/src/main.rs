use std::{net::UdpSocket, str};

fn main() {
    let ip_addr = "127.0.0.1:8888";
    let remote_addr = "127.0.0.1:9999";
    let host = UdpSocket::bind(ip_addr).unwrap();

    host.set_broadcast(true).unwrap();

    let mut buf = [0 as u8; 255];
    match host.recv(&mut buf) {
        Ok(res) => {
            println!("Receive a message");
            let received_mess = str::from_utf8(&buf[..res]).unwrap();
            println!("{:?}", received_mess);

            let response = "Hello Client";
            host.send_to(
                response.as_bytes(),
                remote_addr,
            ).unwrap();
        },
        Err(e) => {
            println!("{:?}", e);
        }

    }
}