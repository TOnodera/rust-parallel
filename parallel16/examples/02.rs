use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("test.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    buffer.iter().map(|u| {
        println!("{:?}", *u as char);
    });
    Ok(())
}
