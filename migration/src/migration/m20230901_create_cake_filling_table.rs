use super::m20230901_create_cake_table::Cake;
use super::m20230901_create_filling_table::Filling;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230901_create_cake_filling_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .if_not_exists()
            .table(CakeFilling::Table)
            .col(
                ColumnDef::new(CakeFilling::FillingId)
                    .comment("filling id")
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CakeFilling::CakeId)
                    .comment("cake id")
                    .not_null()
                    .integer(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_cake_filling_cake_id")
                    .from(CakeFilling::Table, CakeFilling::CakeId)
                    .to(Cake::Table, Cake::Id),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_cake_filling_filling_id")
                    .from(CakeFilling::Table, CakeFilling::FillingId)
                    .to(Filling::Table, Filling::Id),
            )
            .primary_key(
                Index::create()
                    .col(CakeFilling::CakeId)
                    .col(CakeFilling::FillingId),
            );

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut drop_table = Table::drop();
        drop_table.if_exists().table(CakeFilling::Table);
        manager.drop_table(drop_table).await
    }
}

#[derive(Iden)]
pub enum CakeFilling {
    Table,
    CakeId,
    FillingId,
}
