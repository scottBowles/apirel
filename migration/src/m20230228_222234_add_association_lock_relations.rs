use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add to the association table a lock_time and lock_user_id column
        // where lock_time is the time the lock was set and lock_user_id is the
        // user who set the lock
        manager
            .alter_table(
                Table::alter()
                    .table(Association::Table)
                    .add_column(
                        &mut ColumnDef::new(Association::LockTime)
                            .timestamp_with_time_zone()
                            .to_owned(),
                    )
                    .add_column(&mut ColumnDef::new(Association::LockUserId).uuid().to_owned())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_association_lock_user_id")
                            .from_tbl(Association::Table)
                            .from_col(Association::LockUserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Association::Table)
                    .drop_column(Association::LockTime)
                    .drop_column(Association::LockUserId)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Association {
    Table,
    LockTime,
    LockUserId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
