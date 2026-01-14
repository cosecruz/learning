use verb_beta::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("I am Verb- actions speaks louder than words");

    connect_api().await;
}
