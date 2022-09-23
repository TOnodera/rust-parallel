use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {});
    }
    Ok(())
}
