use std::{net::UdpSocket, str};

fn main() {
    let addr = "127.0.0.1:9999";
    let target = "127.0.0.1:8888";

    let host = UdpSocket::bind(addr).unwrap();

    let mess = "Hello Server";

    //host.connect(target);

    match host.send_to(mess.as_bytes(), target) {
        Ok(_) => {
            println!("Message sent");
        },
        Err(err) => {
            println!("{:?}", err);
        }
    };

    let mut buf =  [0 as u8; 255];

    match host.recv(&mut buf) {
        Ok(rec) => {
            println!("Resonse: {:?}", str::from_utf8(&buf[..rec]).unwrap());
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
