mod api;

/// connect_api
pub async fn connect_api() {
    println!("[connecting_api]");

    api::serve_api().await;
}
