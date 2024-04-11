use secrecy::Secret;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub logined_at: NaiveDateTime,
    pub status: UserStatus,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserData {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub status: UserStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub status: UserStatus,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordData {
    pub old_password: Secret<String>,
    pub new_password: Secret<String>,
}

pub struct User {
    pub name: String,
    pub nickname: Option<String>,
    pub email: String,
    pub password: Secret<String>,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub logined_at: Option<NaiveDateTime>,
    pub status: UserStatus,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum UserStatus {
    Active = 0,
    Inactive = 1,
    Suspended = 2,
    Deleted = 3,
}

impl UserStatus {
    pub fn suspend(&mut self) {
        if self == &UserStatus::Active {
            *self = UserStatus::Suspended;
        }
    }

    pub fn activate(&mut self) -> anyhow::Result<(), &'static str> {
        match self {
            UserStatus::Deleted { .. } => return Err("can't active a deleted user"),
            _ => *self = UserStatus::Active,
        }

        Ok(())
    }

    pub fn delete(&mut self) {
        if let UserStatus::Deleted { .. } = self {
            return;
        }
        *self = UserStatus::Deleted
    }

    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, UserStatus::Suspended { .. })
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self, UserStatus::Deleted { .. })
    }
}

impl TryFrom<i8> for UserStatus {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UserStatus::Active),
            1 => Ok(UserStatus::Inactive),
            2 => Ok(UserStatus::Suspended),
            3 => Ok(UserStatus::Deleted),
            _ => Err(()),
        }
    }
}

impl UserStatus {
    pub fn to_i8(&self) -> i8 {
        match self {
            UserStatus::Active => 0,
            UserStatus::Inactive => 1,
            UserStatus::Suspended => 2,
            UserStatus::Deleted => 3,
        }
    }

    pub fn from_i8(value: i8) -> Self {
        match value {
            0 => UserStatus::Active,
            1 => UserStatus::Inactive,
            2 => UserStatus::Suspended,
            3 => UserStatus::Deleted,
            _ => UserStatus::Active,
        }
    }
}
