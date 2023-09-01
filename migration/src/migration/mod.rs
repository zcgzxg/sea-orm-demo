use sea_orm_migration::prelude::*;
mod m20230822_create_bakery_table;
mod m20230823_create_chef_table;
mod m20230901_create_cake_filling_table;
mod m20230901_create_cake_table;
mod m20230901_create_filling_table;
mod m20230901_create_fruit_table;

pub struct Migrator {}

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230822_create_bakery_table::Migration),
            Box::new(m20230823_create_chef_table::Migration),
            Box::new(m20230901_create_cake_table::Migration),
            Box::new(m20230901_create_fruit_table::Migration),
            Box::new(m20230901_create_filling_table::Migration),
            Box::new(m20230901_create_cake_filling_table::Migration),
        ]
    }
}
