mod starwars;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Request, Response, Schema};
use axum::response::IntoResponse;
use axum::{
    extract::Extension, handler::get, response::Html, route, routing::RoutingDsl,
    AddExtensionLayer, Json,
};
use starwars::{QueryRoot, StarWars, StarWarsSchema};

async fn graphql_handler(schema: Extension<StarWarsSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    let app = route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
