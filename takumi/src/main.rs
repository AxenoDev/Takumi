use minecraft_packet::packets::{handshaking, login, status};
use minecraft_packet::{Connection, ConnectionState, ProtocolError};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:25565";
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind on port 25565");

    println!("Takumi listening on {addr}");

    loop {
        let (socket, client_addr) = listener.accept().await.unwrap();
        println!("connection from {client_addr}");

        tokio::spawn(async move {
            if let Err(err) = handle_connection(socket).await {
                eprintln!("{client_addr}: {err}");
            }
        });
    }
}

async fn handle_connection(socket: tokio::net::TcpStream) -> Result<(), ProtocolError> {
    let mut conn = Connection::new(socket);
    let mut state = ConnectionState::Handshaking;

    loop {
        let raw = conn.receive().await?;

        match state {
            ConnectionState::Handshaking => {
                let intent = handshaking::handle(raw)?;
                state = ConnectionState::from(intent);
            }

            ConnectionState::Status => {
                if status::handle(&mut conn, raw).await? {
                    break;
                }
            }

            ConnectionState::Login => {
                login::handle(&mut conn, raw).await?;
            }

            ConnectionState::Transfer => {
                return Err(ProtocolError::UnknownPacket { id: raw.id });
            }
        }
    }

    Ok(())
}
