use crate::{
    app::features::{profile::entities::Profile, user::entities::User},
    error::AppError,
    utils::db::DbPool,
};

use super::entities::Comment;

pub trait CommentRepository: Send + Sync + 'static {
    fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<(Comment, Profile), AppError>;
}

#[derive(Clone)]
pub struct CommentRepositoryImpl {
    pool: DbPool,
}
