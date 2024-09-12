use std::sync::Arc;

use crate::app::features::{
    article::{
        presenters::ArticlePresenterImpl, repositories::ArticleRepositoryImpl,
        usecases::ArticleUsecase,
    },
    profile::{
        presenters::ProfilePresenterImpl, repositories::ProfileRepositoryImpl,
        usecases::ProfileUsecase,
    },
    tag::{presenters::TagPresenterImpl, repositories::TagRepositoryImpl, usecases::TagUsecase},
    user::{
        presenters::UserPresenterImpl, repositories::UserRepositoryImpl, usercases::UserUsecase,
    },
};

use super::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {
    /**
     * Tag
     */
    pub tag_repository: TagRepositoryImpl,
    pub tag_presenter: TagPresenterImpl,
    pub tag_usecase: TagUsecase,

    /**
     * User
     */
    pub user_repository: UserRepositoryImpl,
    pub user_presenter: UserPresenterImpl,
    pub user_usecase: UserUsecase,
    /*
     * Profile
     */
    pub profile_repository: ProfileRepositoryImpl,
    pub profile_presenter: ProfilePresenterImpl,
    pub profile_usecase: ProfileUsecase,

    // Article
    pub article_repository: ArticleRepositoryImpl,
    pub article_presenter: ArticlePresenterImpl,
    pub article_usecase: ArticleUsecase,
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let tag_repository = TagRepositoryImpl::new(pool.clone());
        let user_repository = UserRepositoryImpl::new(pool.clone());
        let profile_repository = ProfileRepositoryImpl::new(pool.clone());
        let article_repository = ArticleRepositoryImpl::new(pool.clone());

        // Presenter
        let tag_presenter = TagPresenterImpl::new();
        let user_presenter = UserPresenterImpl::new();
        let profile_presenter = ProfilePresenterImpl::new();
        let article_presenter = ArticlePresenterImpl::new();

        // Usecase
        let tag_usecase = TagUsecase::new(
            Arc::new(tag_repository.clone()),
            Arc::new(tag_presenter.clone()),
        );
        let user_usecase = UserUsecase::new(
            Arc::new(user_repository.clone()),
            Arc::new(user_presenter.clone()),
        );
        let profile_usecase = ProfileUsecase::new(
            Arc::new(profile_repository.clone()),
            Arc::new(user_repository.clone()),
            Arc::new(profile_presenter.clone()),
        );
        let article_usecase = ArticleUsecase::new(
            Arc::new(article_repository.clone()),
            Arc::new(article_presenter.clone()),
        );
        Self {
            // Tag
            tag_repository,
            tag_presenter,
            tag_usecase,

            // User
            user_repository,
            user_presenter,
            user_usecase,

            // Profile
            profile_repository,
            profile_presenter,
            profile_usecase,

            // Artcile
            article_repository,
            article_presenter,
            article_usecase,
        }
    }
}
