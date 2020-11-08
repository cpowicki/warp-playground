use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let (_, server) =
        warp::serve(hello).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
            if let Err(e) = tokio::signal::ctrl_c().await {
                println!("Error during shutdown {:?}", e);
            }
            println!("Shutting down...");
        });

    tokio::task::spawn(server).await;
}
