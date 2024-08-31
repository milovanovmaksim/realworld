use crate::{app::drivers::middlewares::state::AppState, utils::api::ApiResponse};
use actix_web::web;

pub async fn index(state: web::Data<AppState>) -> ApiResponse {
    state.di_container.tag_usecase.fetch_tags()
}
