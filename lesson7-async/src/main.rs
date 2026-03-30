mod basics;

#[tokio::main]
async fn main() {
    basics::run().await;
}
