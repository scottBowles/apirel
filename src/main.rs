use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    *,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[post("/")]
async fn index(
    // Schema now accessible here
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/")]
async fn index_playground() -> HttpResponse {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(index_playground)
            .app_data(web::Data::new(schema.clone()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "world"
    }
}
