pub use sea_orm_migration::prelude::*;

mod m20220719_133721_create_group_table;
mod m20220719_133751_create_user_table;
mod m20220720_152913_seed_users_and_groups;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220719_133721_create_group_table::Migration),
            Box::new(m20220719_133751_create_user_table::Migration),
            Box::new(m20220720_152913_seed_users_and_groups::Migration),
        ]
    }
}
