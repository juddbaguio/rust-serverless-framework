use std::{env::var, sync::Arc};

use axum::{extract::State, response::Redirect, routing::get, Router};

fn app() -> Router {
    let nickname = Arc::new(match var("NICKNAME") {
        Ok(val) => val,
        _ => "".to_string(),
    });

    Router::new()
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
        .with_state(nickname)
}

#[tokio::main]
async fn main() {
    let route_app = app();
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(route_app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_redirect() {
        let app = app();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();
    }
}
