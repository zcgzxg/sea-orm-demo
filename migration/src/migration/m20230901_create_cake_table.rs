use sea_orm_migration::{async_trait::async_trait, prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230901_create_cake_table"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(Cake::Table)
            .col(
                ColumnDef::new(Cake::Id)
                    .comment("cake id")
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Cake::Name)
                    .comment("cake name")
                    .char_len(32)
                    .not_null(),
            );

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.table(Cake::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum Cake {
    Table,
    Id,
    Name,
}
