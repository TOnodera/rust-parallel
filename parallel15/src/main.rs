use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1,2,3];

    let h = task::spawn(async move{
        println!("{:?}", v);
    });

    h.await.unwrap();
}
