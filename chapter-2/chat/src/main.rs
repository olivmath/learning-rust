use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let localhost = "localhost:3000";
    let listener = TcpListener::bind(localhost).await.unwrap();

    println!("✅ Server started!");
    println!("🌎 http://{localhost}");

    let (mut socket, _addr) = listener.accept().await.unwrap();
    println!("+1 New connection");
    loop {

        let mut buffer = [0u8; 1024];
        let bytes_read_length: usize = socket.read(&mut buffer).await.unwrap();
        socket
            .write_all(&buffer[0..bytes_read_length])
            .await
            .unwrap();
    }
}
