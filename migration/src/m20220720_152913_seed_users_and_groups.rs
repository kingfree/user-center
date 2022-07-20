use entity::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let admin = group::ActiveModel {
            name: Set("admin".to_owned()),
            level: Set(1),
            ..Default::default()
        }
        .insert(db)
        .await?;
        let common = group::ActiveModel {
            name: Set("common".to_owned()),
            level: Set(0),
            ..Default::default()
        }
        .insert(db)
        .await?;

        user::ActiveModel {
            name: Set("root".to_owned()),
            password: Set(util::encode_password("root")),
            group_id: Set(admin.id),
            ..Default::default()
        }
        .insert(db)
        .await?;
        user::ActiveModel {
            name: Set("user".to_owned()),
            password: Set(util::encode_password("password")),
            group_id: Set(common.id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        group::Entity::delete_many()
            .filter(group::Column::Name.eq("admin"))
            .exec(db)
            .await?;
        group::Entity::delete_many()
            .filter(group::Column::Name.eq("common"))
            .exec(db)
            .await?;
        user::Entity::delete_many()
            .filter(user::Column::Name.eq("root"))
            .exec(db)
            .await?;
        user::Entity::delete_many()
            .filter(user::Column::Name.eq("user"))
            .exec(db)
            .await?;
        Ok(())
    }
}
