use actix_web::{web, HttpResponse, Responder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::graphql::GraphQLSchema;
use serde_json;

// Health
// ------

/// Handle health check requests
pub async fn health_handler() -> impl Responder {
    serde_json::json!({
        "code": "200",
        "success": true,
    })
    .to_string()
}

// GraphQL
// ------
pub async fn index(schema: web::Data<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}
