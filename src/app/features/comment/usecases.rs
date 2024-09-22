use super::{presenters::CommentPresenter, repositories::CommentRepository};
use crate::{app::features::user::entities::User, error::AppError};
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct CommentUsecase {
    comment_repository: Arc<dyn CommentRepository>,
    comment_presenter: Arc<dyn CommentPresenter>,
}

impl CommentUsecase {
    pub fn new(
        comment_repository: Arc<dyn CommentRepository>,
        comment_presenter: Arc<dyn CommentPresenter>,
    ) -> Self {
        Self {
            comment_repository,
            comment_presenter,
        }
    }

    pub fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .comment_repository
            .create_comment(body, article_title_slug, author)?;
        let res = self.comment_presenter.to_single_json(result);
        Ok(res)
    }
}
