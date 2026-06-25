mod protocol;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use protocol::reader::PacketReader;
use crate::protocol::packets::handshake::HandshakePacket;
use crate::protocol::packets::transfer::TransferPacket;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:25565";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to binding on port 25565");

    println!("Takumi binding on port 25565");

    loop {
        let (mut socket, client_addr) = listener.accept().await.unwrap();
        println!("new connexion from {}", client_addr);

        tokio::spawn(async move {
            let mut buf = [0u8; 4096];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("{} disconnected", client_addr);
                        break;
                    }
                    Ok(n) => {
                        println!("received {} bytes", n);
                        println!("{:02X?}", &buf[..n]);

                        let mut reader = PacketReader::new(&buf[..n]);

                        let packet_length = match reader.read_varint() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("{e}");
                                continue;
                            }
                        };

                        let packet_id = match reader.read_varint() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("{e}");
                                continue;
                            }
                        };

                        match packet_id {
                            0x00 => {
                                let handshake = match HandshakePacket::decode(&mut reader) {
                                    Ok(packet) => packet,
                                    Err(e) => {
                                        eprintln!("handshake error: {e}");
                                        continue;
                                    }
                                };

                                println!("{:#?}", handshake);
                            }

                            0x7A => {
                                let transfer = match TransferPacket::decode(&mut reader) {
                                    Ok(packet) => packet,
                                    Err(e) => {
                                        eprintln!("transfer error: {e}");
                                        continue;
                                    }
                                };

                                println!("{:#?}", transfer)
                            }

                            _ => {
                                println!("unknown packet");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("read error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}