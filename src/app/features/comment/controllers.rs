use actix_web::{web, HttpRequest};

use crate::{app::drivers::middlewares::state::AppState, utils::api::ApiResponse};

use super::request;

type ArticleIdSlug = String;

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> ApiResponse {
}
