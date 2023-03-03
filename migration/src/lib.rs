pub use sea_orm_migration::prelude::*;

mod m20230208_000001_create_user_table;
mod m20230228_192033_create_association_table;
mod m20230228_222234_add_association_lock_relations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230208_000001_create_user_table::Migration),
            Box::new(m20230228_192033_create_association_table::Migration),
            Box::new(m20230228_222234_add_association_lock_relations::Migration),
        ]
    }
}
