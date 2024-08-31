use std::sync::Arc;

use crate::app::features::tag::{
    presenters::TagPresenterImpl, repositories::TagRepositoryImpl, usecases::TagUsecase,
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
}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        // Repository
        let tag_repository = TagRepositoryImpl::new(pool.clone());

        // Presenter
        let tag_presenter = TagPresenterImpl::new();

        // Usecase
        let tag_usecase = TagUsecase::new(
            Arc::new(tag_repository.clone()),
            Arc::new(tag_presenter.clone()),
        );
        Self {
            // Tag
            tag_repository,
            tag_presenter,
            tag_usecase,
        }
    }
}
