use crate::{error::AppError, utils::db::DbPool};

use super::entities::User;

type Token = String;

pub trait UserRepository: Send + Sync + 'static {
    fn signin(&self, email: &str, naive_password: &str) -> Result<(User, Token), AppError>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn signin(&self, email: &str, naive_password: &str) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        User::signin(conn, email, naive_password)
    }
}
