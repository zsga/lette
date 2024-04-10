use sqlx::MySqlPool;

pub struct UserRepo {
    pub pool: MySqlPool,
}

impl UserRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}
