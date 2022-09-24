use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = File::create("write_test04.txt").await?;
    buffer.write_all(b"some bytes").await?;
    Ok(())
}
