use axum::{routing::get, Router};

#[tokio::main]

async fn main() {
    let app: Router = Router::new().route("/", get(hello_world));
    axum::Server::bind(&"0.0.0.0:3344".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String {
    "Hello World".to_owned()
}
