mod migration;

use clap::{Parser, ValueEnum};
use migration::Migrator;
use sea_orm_demo::create_example_conn;
use sea_orm_migration::prelude::*;

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

#[async_std::main]
async fn main() -> Result<(), sea_orm::error::DbErr> {
    let cli = Cli::parse();
    let db = &create_example_conn().await?;

    match cli.mode {
        Mode::Refresh => Migrator::refresh(db).await,
        Mode::Up => Migrator::up(db, cli.steps).await,
        Mode::Status => Migrator::status(db).await,
        Mode::Fresh => Migrator::fresh(db).await,
        Mode::Down => Migrator::down(db, cli.steps).await,
    }
}
