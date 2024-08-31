use crate::{error::AppError, utils::db::DbPool};

use super::entities::Tag;

pub trait TagRepository: Send + Sync + 'static {
    fn fetch_tags(&self) -> Result<Vec<Tag>, AppError>;
}

#[derive(Clone)]
pub struct TagRepositoryImpl {
    pool: DbPool,
}

impl TagRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
impl TagRepository for TagRepositoryImpl {
    fn fetch_tags(&self) -> Result<Vec<Tag>, AppError> {
        let conn = &mut self.pool.get()?;
        Tag::fetch(conn)
    }
}
