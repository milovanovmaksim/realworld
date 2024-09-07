use actix_web::{web, HttpRequest};

use crate::{
    app::drivers::middlewares::{auth, state::AppState},
    utils::api::ApiResponse,
};

use super::requests;

pub async fn signin(state: web::Data<AppState>, form: web::Json<requests::Signin>) -> ApiResponse {
    state
        .di_container
        .user_usecase
        .signin(&form.user.email, &form.user.password)
}

pub async fn signup(state: web::Data<AppState>, form: web::Json<requests::Signup>) -> ApiResponse {
    state.di_container.user_usecase.signup(
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )
}

pub async fn me(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    state.di_container.user_usecase.get_token(&current_user)
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<requests::Update>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    state.di_container.user_usecase.update_user(
        current_user.id,
        super::entities::UpdateUser {
            email: form.user.email.clone(),
            username: form.user.username.clone(),
            password: form.user.password.clone(),
            image: form.user.image.clone(),
            bio: form.user.bio.clone(),
        },
    )
}
