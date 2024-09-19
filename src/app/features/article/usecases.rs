use super::{
    presenters::ArticlePresenter,
    repositories::{
        ArticleRepository, FetchArticlesRepositoryInput, FetchFollowingArticlesRepositoryInput,
    },
};
use crate::{app::features::user::entities::User, error::AppError};
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct ArticleUsecase {
    article_repository: Arc<dyn ArticleRepository>,
    article_presenter: Arc<dyn ArticlePresenter>,
}

impl ArticleUsecase {
    pub fn new(
        article_repository: Arc<dyn ArticleRepository>,
        article_presenter: Arc<dyn ArticlePresenter>,
    ) -> Self {
        Self {
            article_repository,
            article_presenter,
        }
    }

    pub fn fetch_articles(
        &self,
        params: FetchArticlesUsecaseInput,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) =
            self.article_repository
                .fetch_articles(FetchArticlesRepositoryInput {
                    tag: params.tag,
                    author: params.author,
                    favorited: params.favorited,
                    offset: params.offset,
                    limit: params.limit,
                })?;
        let res = self.article_presenter.to_multi_json(list, count);
        Ok(res)
    }

    pub fn fetch_article_by_slug(
        &self,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let result = self
            .article_repository
            .fetch_article_by_slug(article_title_slug)?;
        let res = self.article_presenter.to_single_json(result);
        Ok(res)
    }

    pub fn fetch_following_articles(
        &self,
        user: User,
        offset: i64,
        limit: i64,
    ) -> Result<HttpResponse, AppError> {
        let (list, count) = self.article_repository.fetch_following_articles(
            &FetchFollowingArticlesRepositoryInput {
                current_user: user,
                offset,
                limit,
            },
        )?;
        let res = self.article_presenter.to_multi_json(list, count);
        Ok(res)
    }
}

pub struct FetchArticlesUsecaseInput {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}
