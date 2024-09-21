use std::sync::Arc;

use actix_web::HttpResponse;

use crate::{
    app::features::{
        article::repositories::{ArticleRepository, FetchArticleRepositoryInput},
        user::entities::User,
    },
    error::AppError,
};

use super::{presentres::FavoritePresenter, repository::FavoriteRepository};

#[derive(Clone)]
pub struct FavoriteUsecase {
    favorite_repository: Arc<dyn FavoriteRepository>,
    favorite_presenter: Arc<dyn FavoritePresenter>,
    article_repository: Arc<dyn ArticleRepository>,
}

impl FavoriteUsecase {
    pub fn new(
        favorite_repository: Arc<dyn FavoriteRepository>,
        favorite_presenter: Arc<dyn FavoritePresenter>,
        article_repository: Arc<dyn ArticleRepository>,
    ) -> Self {
        Self {
            favorite_repository,
            favorite_presenter,
            article_repository,
        }
    }

    pub fn favorite_article(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let article = self
            .favorite_repository
            .favorite_article(user.clone(), article_title_slug)?;

        let result = self
            .article_repository
            .fetch_article(&FetchArticleRepositoryInput {
                article_id: article.id,
                current_user: user,
            })?;
        let res = self.favorite_presenter.to_single_json(result);
        Ok(res)
    }

    pub fn unfavorite_article(
        &self,
        user: User,
        article_title_slug: String,
    ) -> Result<HttpResponse, AppError> {
        let article = self
            .favorite_repository
            .unfavorite_article(user.clone(), article_title_slug)?;
        let result = self
            .article_repository
            .fetch_article(&FetchArticleRepositoryInput {
                article_id: article.id,
                current_user: user,
            })?;
        let res = self.favorite_presenter.to_single_json(result);
        Ok(res)
    }
}
