use actix_web::web;

use crate::{app::drivers::middlewares::state::AppState, utils::api::ApiResponse};

use super::requests;

pub async fn signin(state: web::Data<AppState>, form: web::Json<requests::Signin>) -> ApiResponse {
    state
        .di_container
        .user_usecase
        .signin(&form.user.email, &form.user.password)
}
