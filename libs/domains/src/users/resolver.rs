use crate::prelude::*;
use async_graphql::{ComplexObject, Context, Object};
use sea_orm::{prelude::Uuid, *};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    // For finding all users
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        User::find().all(db).await
    }

    // For finding one user by id
    pub async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        User::find_by_id(id).one(db).await
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
