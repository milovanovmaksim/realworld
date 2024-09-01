use std::sync::Arc;

use crate::app::features::{
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
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let tag_repository = TagRepositoryImpl::new(pool.clone());
        let user_repository = UserRepositoryImpl::new(pool.clone());

        // Presenter
        let tag_presenter = TagPresenterImpl::new();
        let user_presenter = UserPresenterImpl::new();

        // Usecase
        let tag_usecase = TagUsecase::new(
            Arc::new(tag_repository.clone()),
            Arc::new(tag_presenter.clone()),
        );
        let user_usecase = UserUsecase::new(
            Arc::new(user_repository.clone()),
            Arc::new(user_presenter.clone()),
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
        }
    }
}
