use super::{presenters::ProfilePresenter, repositories::ProfileRepository};
use crate::{
    app::features::user::{entities::User, repositories::UserRepository},
    error::AppError,
};
use actix_web::HttpResponse;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProfileUsecase {
    user_repository: Arc<dyn UserRepository>,
    profile_repository: Arc<dyn ProfileRepository>,
    presenter: Arc<dyn ProfilePresenter>,
}

type Type = HttpResponse;

impl ProfileUsecase {
    pub fn new(
        profile_repository: Arc<dyn ProfileRepository>,
        user_repository: Arc<dyn UserRepository>,
        presenter: Arc<dyn ProfilePresenter>,
    ) -> Self {
        Self {
            user_repository,
            profile_repository,
            presenter,
        }
    }

    pub fn fetch_profile_by_name(
        &self,
        current_user: &User,
        username: &str,
    ) -> Result<Type, AppError> {
        let profile = self
            .profile_repository
            .fetch_profile_by_name(current_user, username)?;
        Ok(self.presenter.to_json(profile))
    }

    pub fn follow_user(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<HttpResponse, AppError> {
        let profile = self
            .user_repository
            .follow_user(current_user, target_username)?;
        Ok(self.presenter.to_json(profile))
    }

    pub fn unfollow_user(
        &self,
        current_user: &User,
        targer_username: &str,
    ) -> Result<HttpResponse, AppError> {
        let profile = self
            .user_repository
            .unfollow_user(current_user, targer_username)?;
        Ok(self.presenter.to_json(profile))
    }
}
