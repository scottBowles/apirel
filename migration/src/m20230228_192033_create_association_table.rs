use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Association::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Association::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Association::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Association::Description).string())
                    .col(ColumnDef::new(Association::ThumbnailId).string())
                    .col(ColumnDef::new(Association::MarkdownNotes).string())
                    .col(ColumnDef::new(Association::ImageIds).array(ColumnType::String(None)))
                    .col(
                        ColumnDef::new(Association::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Association::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Association::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Association {
    Table,
    Id,
    Name,
    Description,
    ThumbnailId,
    MarkdownNotes,
    ImageIds,
    CreatedAt,
    UpdatedAt,
}
