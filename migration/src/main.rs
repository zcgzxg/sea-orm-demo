mod migration;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use migration::Migrator;
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::prelude::*;
use std::env;
use tracing;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 执行模式
    #[arg(short, value_enum)]
    mode: Mode,
    /// 执行的步数，默认为全部
    #[arg(short)]
    steps: Option<u32>,
}

/// 迁移模式
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    /// 回滚所有已应用的迁移，然后重新执行迁移
    Refresh,
    /// 删除所有表，然后重新执行迁移
    Fresh,
    /// 执行待运行的迁移，可指定steps
    Up,
    /// 检查迁移状态
    Status,
    /// 回滚迁移，可指定steps
    Down,
}

lazy_static! {
    static ref CONN_URL: String = env::var("DATABASE_URL").unwrap();
}

#[async_std::main]
async fn main() -> Result<(), sea_orm::error::DbErr> {
    lazy_static::initialize(&CONN_URL);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let cli = Cli::parse();
    let mut connect_opts = ConnectOptions::new(&CONN_URL.to_owned());
    connect_opts.max_connections(200);
    let db = &Database::connect(connect_opts).await?;

    match cli.mode {
        Mode::Refresh => Migrator::refresh(db).await,
        Mode::Up => Migrator::up(db, cli.steps).await,
        Mode::Status => Migrator::status(db).await,
        Mode::Fresh => Migrator::fresh(db).await,
        Mode::Down => Migrator::down(db, cli.steps).await,
    }
}
