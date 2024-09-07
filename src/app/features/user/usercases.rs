use std::sync::Arc;

use actix_web::HttpResponse;

use crate::error::AppError;

use super::{presenters::UserPresenter, repositories::UserRepository};

#[derive(Clone)]
pub struct UserUsecase {
    user_repository: Arc<dyn UserRepository>,
    user_presenter: Arc<dyn UserPresenter>,
}
impl UserUsecase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_presenter: Arc<dyn UserPresenter>,
    ) -> Self {
        Self {
            user_repository,
            user_presenter,
        }
    }

    pub fn signin(&self, email: &str, password: &str) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signin(email, password)?;
        let res = self.user_presenter.to_json(user, token);
        Ok(res)
    }

    pub fn signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<HttpResponse, AppError> {
        let (user, token) = self.user_repository.signup(email, username, password)?;
        let res = self.user_presenter.to_json(user, token);
        Ok(res)
    }
}
