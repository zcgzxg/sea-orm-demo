use super::m20230901_create_cake_table::Cake;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230901_create_fruit_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(Fruit::Table)
            .col(
                ColumnDef::new(Fruit::Id)
                    .comment("fruit id")
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Fruit::Name)
                    .comment("fruit name")
                    .char_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Fruit::CakeId)
                    .comment("fruit_cake id")
                    .not_null()
                    .integer(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_fruit_cake_id")
                    .from(Fruit::Table, Fruit::CakeId)
                    .to(Cake::Table, Cake::Id),
            );

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.if_exists().table(Fruit::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum Fruit {
    Table,
    Id,
    Name,
    CakeId,
}
