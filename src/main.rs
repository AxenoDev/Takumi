use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
                        let mut pos = 0;

                        let length = match read_varint(&buf[..n], &mut pos) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("varint error: {}", e);
                                continue;
                            }
                        };
                        let packet_id =  match read_varint(&buf[..n], &mut pos) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("varint error: {}", e);
                                continue;
                            }
                        };
                        
                        let protocol_version = match read_varint(&buf[..n], &mut pos) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("varint error: {}", e);
                                continue;
                            }
                        };
                        
                        println!("received {} bytes", n);
                        println!("{:02X?}", &buf[..n]);
                        println!("length={length}");
                        println!("packet_id=0x{packet_id:X}");
                        println!("Protocol version: {protocol_version}");
                    }
                    Err(e) => {
                        eprintln!("read errorr {}", e);
                        break;
                    }
                }
            }
        });
    }
}

fn read_varint(buf: &[u8], pos: &mut usize) -> Result<i32, &'static str> {
    let mut num_read = 0;
    let mut result = 0i32;

    loop {
        if *pos >= buf.len() {
            return Err("Unexpected eof");
        }

        let byte = buf[*pos];
        *pos += 1;

        let value = (byte & 0x7F) as i32;
        result |= value << (7 * num_read);

        num_read += 1;

        if (num_read > 5) {
            return Err("varint too big");
        }

        if (byte & 0x80) == 0 {
            break;
        }
    }

    Ok(result)
}
