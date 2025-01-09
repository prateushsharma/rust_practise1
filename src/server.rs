use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(0) => return, // Connection closed
                Ok(n) => {
                    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
                    socket.write_all(&buf[..n]).await.unwrap();
                }
                Err(e) => {
                    println!("Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}
