// src/schema.rs

use crate::prelude::*;
use async_graphql::{ComplexObject, Context, Object};
use sea_orm::{prelude::Uuid, *};

#[derive(Default)]
pub struct AssociationsQuery;

#[Object]
impl AssociationsQuery {
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
