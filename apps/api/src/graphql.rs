use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use sea_orm::DatabaseConnection;

use apirel_domains::{associations::resolver::AssociationsQuery, users::resolver::UsersQuery};

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, AssociationsQuery);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Create a new GraphQL schema
pub fn create_schema(db: DatabaseConnection) -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
