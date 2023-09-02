use entities::cake;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
use sea_orm_demo::create_example_conn;

#[async_std::main]
async fn main() -> Result<(), DbErr> {
    let db = &create_example_conn().await?;
    let cheese = cake::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("cheese".to_owned()),
    };

    let model = cheese.insert(db).await?;
    println!("inserted value: {:?}", model);
    Ok(())
}
