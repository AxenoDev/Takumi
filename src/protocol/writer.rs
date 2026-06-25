use tokio::io::AsyncWriteExt;

pub fn write_varint(mut value: i32) -> Vec<u8> {
    let mut out = Vec::new();

    loop {
        if (value & !0x7F) == 0 {
            out.push(value as u8);
            return out;
        }

        out.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }
}

async fn send_status_response(
    socket: &mut tokio::net::TcpStream,
) -> std::io::Result<()> {
    let json = r#"{
        "version":{
            "name":"26.2",
            "protocol":776
        },
        "players":{
            "max":100,
            "online":0
        },
        "description":{
            "text":"§6Takumi Server"
        }
    }"#;

    let mut packet = Vec::new();

    packet.extend(write_varint(0));

    packet.extend(write_varint(json.len() as i32));
    packet.extend(json.as_bytes());

    let mut framed = Vec::new();
    framed.extend(write_varint(packet.len() as i32));
    framed.extend(packet);

    socket.write_all(&framed).await
}