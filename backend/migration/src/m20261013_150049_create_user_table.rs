use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261013_150049_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table: TableCreateStatement = Table::create()
            .table("users")
            .if_not_exists()
            .col(pk_uuid("id"))
            .col(uuid("sub"))
            .col(string("name"))
            .col(string("email"))
            .col(timestamp_with_time_zone("created_at"))
            .take();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users").take())
            .await
    }
}
