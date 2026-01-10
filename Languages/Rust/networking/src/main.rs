use networking::{echo_server, lb_echo_server};

#[tokio::main]
async fn main() {
    // echo_server::connect();
    if let Err(e) = lb_echo_server::connect().await {
        eprintln!("Error: {}", e)
    }
}
