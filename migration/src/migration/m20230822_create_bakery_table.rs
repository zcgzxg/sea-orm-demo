use sea_orm_migration::{async_trait::async_trait, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230822_create_bakery_table"
    }
}
 
#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(Bakery::Table)
            .col(
                ColumnDef::new(Bakery::Id)
                    .comment("primary id")
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Bakery::Name)
                    .comment("bakery name")
                    .char_len(32)
                    .not_null(),
            )
            .col(ColumnDef::new(Bakery::ProfitMargin).not_null().double());

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.table(Bakery::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum Bakery {
    Table,
    Id,
    Name,
    ProfitMargin,
}
