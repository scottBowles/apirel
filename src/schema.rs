// src/schema.rs

use async_graphql::{ComplexObject, Context, Object};
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
        User::find().all(db).await
    }

    // For finding one user by id
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        User::find_by_id(id).one(db).await
    }

    // For finding all associations
    async fn associations(&self, ctx: &Context<'_>) -> Result<Vec<association::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Association::find().all(db).await
    }

    // For finding one association by id
    async fn association(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<association::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Association::find_by_id(id).one(db).await
    }
}

#[ComplexObject]
impl association::Model {
    async fn lock_user(&self, ctx: &Context<'_>) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        match self.lock_user_id {
            Some(id) => User::find_by_id(id).one(db).await,
            None => Ok(None),
        }
    }
}

#[ComplexObject]
impl user::Model {
    async fn association_locks(&self, ctx: &Context<'_>) -> Result<Vec<association::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Association::find()
            .filter(association::Column::LockUserId.eq(self.id))
            .all(db)
            .await
    }
}
