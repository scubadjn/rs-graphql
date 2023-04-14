use crate::model::QueryRoot;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
mod model;
mod routes;

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema));

    Server::bind(&address().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
