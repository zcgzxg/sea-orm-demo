use sea_orm_migration::{async_trait::async_trait, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230901_create_filling_table"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(Filling::Table)
            .col(
                ColumnDef::new(Filling::Id)
                    .comment("filling id")
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Filling::Name)
                    .comment("filling name")
                    .char_len(32)
                    .not_null(),
            );

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.table(Filling::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum Filling {
    Table,
    Id,
    Name,
}
