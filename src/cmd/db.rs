use anyhow::Ok;
use clap::{Args, Subcommand};
use sqlx::{migrate::MigrateDatabase, MySqlPool};

use crate::conf::Conf;

#[derive(Args, Debug)]
pub struct DbCmd {
    /// Database manager
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initial database and tables
    Init,
    /// Destory database and tables
    Destory,
    /// Run migrate
    Upgrade,
}

pub fn handle(cmd: &DbCmd, conf: &Conf) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            match cmd.subcmd {
                Some(Commands::Init) => init_db(conf).await,
                Some(Commands::Destory) => destory_db(conf).await,
                Some(Commands::Upgrade) => upgrade_db(conf).await,
                None => Ok(()),
            }
        })?;

    Ok(())
}

pub async fn init_db(conf: &Conf) -> anyhow::Result<()> {
    conf.db.save_env()?;

    // let exists = Any::database_exists(&conf.db.url()).await?;
    let exists = sqlx::MySql::database_exists(&conf.db.url()).await?;
    if exists {
        tracing::info!("database exists")
    } else {
        sqlx::MySql::create_database(&conf.db.url()).await?;
        tracing::info!("create database");
    }

    Ok(())
}

pub async fn destory_db(conf: &Conf) -> anyhow::Result<()> {
    let exists = sqlx::MySql::database_exists(&conf.db.url()).await?;
    if exists {
        sqlx::MySql::drop_database(&conf.db.url()).await?;
        tracing::info!("drop database")
    }

    Ok(())
}

pub async fn upgrade_db(conf: &Conf) -> anyhow::Result<()> {
    let exists = sqlx::MySql::database_exists(&conf.db.url()).await?;
    if !exists {
        tracing::error!("Please run `cargo run db init` first");
        return Ok(());
    }

    let pool = MySqlPool::connect_with(conf.db.with_db()).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("Applied all migrates");
    Ok(())
}
