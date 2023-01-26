use std::{env::var, sync::Arc};

use axum::{extract::State, response::Redirect, routing::get, Router};

#[tokio::main]
async fn main() {
    let nickname = Arc::new(match var("NICKNAME") {
        Ok(val) => val,
        _ => "".to_string(),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, world! I am Judd" }))
        .route(
            "/github",
            get(move |State(state): State<Arc<String>>| async {
                let nickname = state;
                return Redirect::permanent(&format!("https://www.github.com/{:?}", nickname));
            }),
        )
        .route(
            "/twitter",
            get(move |State(state): State<Arc<String>>| async {
                let nickname = state;
                return Redirect::permanent(&format!("https://www.twitter.com/{:?}", nickname));
            }),
        )
        .with_state(nickname);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
