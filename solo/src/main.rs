use std::net::UdpSocket;

const GAMECONTROLLER_HEADER: &[u8; 4] = b"RGme";
const GC_PORT: u16 = 3838;

fn main() -> std::io::Result<()> {
    println!("Hello, world! {}", GC_PORT);

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", GC_PORT))?;

    let mut buf = [0u8; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let data = &buf[..number_of_bytes];

        let version = data[4];
        let state = data[10];
        println!(
                "Received {} bytes from {} | Protocol Version: {} | Game State ID: {}",
                number_of_bytes, src_addr, version, state
            );
    }
}
