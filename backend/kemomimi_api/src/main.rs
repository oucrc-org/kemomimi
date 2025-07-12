mod app;
mod routes;

use app::app;

#[tokio::main]
async fn main() {
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, app().await).await.unwrap();
}
