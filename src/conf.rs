use std::{fs::OpenOptions, io::Write};

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};

#[derive(Deserialize, Clone)]
pub struct Conf {
    pub app: ApplicationConfig,
    pub db: DatabaseConfig,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub ssl: bool,
}

impl DatabaseConfig {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
    }

    pub fn without_db(&self) -> MySqlConnectOptions {
        let ssl_mode = if self.ssl {
            MySqlSslMode::Required
        } else {
            MySqlSslMode::Preferred
        };

        MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.name)
    }

    pub fn save_env(&self) -> anyhow::Result<()> {
        let base_path = std::env::current_dir().expect("Failed to determine the current dir");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(base_path.join(".env"))?;

        file.write_all(format!("DATABASE_URL={}", self.url()).as_bytes())?;
        Ok(())
    }
}

pub fn new() -> anyhow::Result<Conf, config::ConfigError> {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

    let c = config::Config::builder()
        .add_source(config::File::with_name("config/base"))
        .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
        .add_source(
            config::Environment::with_prefix("LETTE")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    c.try_deserialize()
}
