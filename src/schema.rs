// src/schema.rs

use async_graphql::{Context, Object};
use sea_orm::{prelude::Uuid, *};

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    // For finding all users
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        println!("db: {:?}", db);
        User::find().all(db).await
    }

    // For finding one user by id
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        User::find_by_id(id).one(db).await
    }
}
