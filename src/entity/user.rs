use secrecy::Secret;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub enum UserStatus {
    Active = 0,
    Inactive = 1,
    Suspended = 2,
    Deleted = 3,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub password: Secret<String>,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub logined_at: Option<NaiveDateTime>,
    pub status: UserStatus,
}

pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub logined_at: NaiveDateTime,
    pub status: UserStatus,
}
