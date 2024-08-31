use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use super::entities::User;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            user: AuthUser {
                email: user.email,
                token,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

pub trait UserPresenter: Send + Sync + 'static {
    fn to_json(&self, user: User, token: String) -> HttpResponse;
}

#[derive(Clone)]
pub struct UserPresenterImpl {}

impl UserPresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl UserPresenter for UserPresenterImpl {
    fn to_json(&self, user: User, token: String) -> HttpResponse {
        let resp_model = UserResponse::from((user, token));
        HttpResponse::Ok().json(resp_model)
    }
}
