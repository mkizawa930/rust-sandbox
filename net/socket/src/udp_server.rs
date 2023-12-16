use std::net::UdpSocket;
use std::io::{stdout, Write};
use std::str;
use log::debug;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address)?;
    println!("start serving: {}", address);
    stdout().flush().unwrap();

    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        server_socket.send_to(&buf, src)?; // 文字列を返す
    }
}
