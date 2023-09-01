use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use std::sync::Once;
use tracing;
use tracing_subscriber;

lazy_static! {
    static ref CONN_URL: String = env::var("DATABASE_URL").unwrap();
}

static START: Once = Once::new();

pub async fn create_example_conn() -> Result<DatabaseConnection, DbErr> {
    lazy_static::initialize(&CONN_URL);
    START.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    });

    let mut connect_opts = ConnectOptions::new(&CONN_URL.to_owned());
    connect_opts.max_connections(200);
    Database::connect(connect_opts).await
}
