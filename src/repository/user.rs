use secrecy::ExposeSecret;
use sqlx::MySqlPool;

use crate::entity::user::CreateUserData;
use crate::error::Result as InnerResult;
use crate::util::hash::generate_hash;

pub struct UserRepo {
    pub pool: MySqlPool,
}

impl UserRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_data: &CreateUserData) -> InnerResult<u64> {
        let password_hash = generate_hash(&user_data.password)?;

        let user_id = sqlx::query!(
            r#"INSERT INTO user(name, email, password) values (?, ?, ?)"#,
            user_data.name,
            user_data.email,
            password_hash.expose_secret(),
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(user_id)
    }
}
