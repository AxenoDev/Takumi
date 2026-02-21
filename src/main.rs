use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565")
        .await
        .expect("Failed to listening port 25565");

    println!("Takumi listening on port 25565");

    loop {
        let (socket, addr) = listener.accept().await.expect("Error accept");

        println!("Client connected: {}", addr);
    }
}
