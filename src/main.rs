use lette::router;

#[tokio::main]
async fn main() {
    let app = router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on: http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();
}
