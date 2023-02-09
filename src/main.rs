use std::{env, time::Duration};

use actix_web::{guard, web, web::Data, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use async_graphql::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

async fn db(db_url: String) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

    Database::connect(opt).await
}

#[derive(SimpleObject, InputObject, Clone, Copy)]
#[graphql(input_name = "MyObjectInput")] // Note: You must use the input_name attribute to define a new name for the input type, otherwise a runtime error will occur.
struct MyObject {
    /// Value a
    a: i32,

    /// Value b
    b: i32,
    // #[graphql(skip)]
    // c: i32,
}

#[derive(SimpleObject, Clone, Copy)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
struct MyObj {
    a: i32,
    b: i32,
}

#[ComplexObject]
impl MyObj {
    async fn c(&self) -> i32 {
        self.a + self.b
    }
}

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    async fn my_objects(&self, ctx: &Context<'_>) -> Vec<MyObject> {
        ctx.data_unchecked::<Vec<MyObject>>().clone()
    }
    async fn my_objs(&self, ctx: &Context<'_>) -> Vec<MyObj> {
        ctx.data_unchecked::<Vec<MyObj>>().clone()
    }
}

struct Mutation;

#[Object]
impl Mutation {
    // mutation to add an object to the my_objects array in the context
    async fn add_my_object(&self, ctx: &Context<'_>, input: MyObject) -> MyObject {
        let mut my_objects = ctx.data_unchecked::<Vec<MyObject>>().clone();
        let new_object = MyObject {
            a: input.a,
            b: input.b,
        };
        my_objects.push(new_object);
        // update the context
        ctx.data_unchecked::<Vec<MyObject>>()
            .clone_from(&&my_objects);
        new_object
    }
}

type RootSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<RootSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = db(db_url).await.unwrap();

    let my_objects: Vec<MyObject> = vec![
        MyObject { a: 1, b: 2 },
        MyObject { a: 3, b: 4 },
        MyObject { a: 5, b: 6 },
    ];

    let my_objs: Vec<MyObj> = vec![
        MyObj { a: 12, b: 25 },
        MyObj { a: 33, b: 46 },
        MyObj { a: 54, b: 67 },
    ];

    // let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(my_objects)
        .data(my_objs)
        .finish();

    println!("GraphQL Endpoint: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
