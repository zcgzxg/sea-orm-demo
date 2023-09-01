use super::m20230822_create_bakery_table::Bakery;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230823_create_chef_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(Chef::Table)
            .col(
                ColumnDef::new(Chef::Id)
                    .comment("chef id")
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Chef::Name)
                    .comment("chef name")
                    .char_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Chef::ContactDetail)
                    .comment("chef contactdetail")
                    .not_null()
                    .json(),
            )
            .col(
                ColumnDef::new(Chef::BakeryId)
                    .comment("chef bakery id")
                    .not_null()
                    .integer(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_chef_bakery_id")
                    .from(Chef::Table, Chef::BakeryId)
                    .to(Bakery::Table, Bakery::Id),
            );

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.if_exists().table(Chef::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum Chef {
    Table,
    Id,
    Name,
    ContactDetail,
    BakeryId,
}
