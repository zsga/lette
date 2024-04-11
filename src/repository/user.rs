use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::entity::user::{CreateUserData, UpdateUserData, UserData, UserStatus};
use crate::error::{Error, Result as InnerResult};
use crate::util::hash::{generate_hash, verify_password};

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

    pub async fn update(&self, user_id: i32, user_data: &UpdateUserData) -> InnerResult<bool> {
        let effect_rows = sqlx::query!(
            r#"
                update user
                set email = coalesce(?, user.email),
                    name = coalesce(?, user.name),
                    nickname = coalesce(?, user.nickname),
                    avatar = coalesce(?, user.avatar)
                where id = ?
            "#,
            user_data.email,
            user_data.name,
            user_data.nickname,
            user_data.avatar,
            user_id,
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(effect_rows == 1)
    }

    pub async fn get(&self, user_id: i32) -> InnerResult<UserData> {
        let user = sqlx::query!(
            r#"select id, name, nickname, email, avatar, created_at, logined_at, status from user where id = ?"#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(UserData {
            id: user_id,
            name: user.name,
            nickname: user.nickname,
            email: user.email,
            avatar: user.avatar,
            created_at: user.created_at,
            logined_at: user.logined_at,
            status: UserStatus::from_i8(user.status.unwrap_or(UserStatus::Active.to_i8())),
        })
    }

    pub async fn check(&self, email: String, password: Secret<String>) -> InnerResult<UserData> {
        let user = sqlx::query!(
            r#"select id, email, name, nickname, avatar, password, created_at, logined_at, status from user where email = ?"#,
            email,
        )
        .fetch_optional(&self.pool)
        .await?;

        if user.is_none() {
            return Err(Error::not_found());
        }

        let user = user.unwrap();
        verify_password(&password, &Secret::new(user.password))?;

        Ok(UserData {
            id: user.id,
            email: user.email,
            name: user.name,
            nickname: user.nickname,
            avatar: user.avatar,
            created_at: user.created_at,
            logined_at: user.logined_at,
            status: UserStatus::from_i8(user.status.unwrap_or(UserStatus::Active.to_i8())),
        })
    }
}
