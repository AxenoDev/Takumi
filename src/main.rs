use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:25565";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to binding on port 25565");

    println!("Takumi binding on port 25565");

    loop {
        let (_socket, client_addr) = listener.accept().await.unwrap();
        println!("new connexion from {}", client_addr);
    }
}
