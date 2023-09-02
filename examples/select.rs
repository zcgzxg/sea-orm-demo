use async_std::stream::StreamExt;
use entities::cake;
use sea_orm::EntityTrait;
use sea_orm_demo::create_example_conn;

#[async_std::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db = &create_example_conn().await?;
    let cheese = cake::Entity::find_by_id(0).one(db).await?;
    println!("cake of id 0: {:?}", cheese);

    println!("first five cakes");
    let mut cakes = cake::Entity::find().stream(db).await?.take(5);
    while let Some(item) = cakes.next().await {
        match item {
            Ok(item) => {
                println!("{:?}", item);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
