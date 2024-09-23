use actix_web::{web, HttpRequest};

use crate::{
    app::drivers::middlewares::{auth, state::AppState},
    utils::api::ApiResponse,
};

use super::request;

type ArticleIdSlug = String;

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    let body = form.comment.body.to_owned();
    state
        .di_container
        .comment_usecase
        .create_comment(body, article_title_slug, current_user)
}

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> ApiResponse {
    let current_user = auth::get_current_user(&req).ok();
    state
        .di_container
        .comment_usecase
        .fetch_comments(&current_user)
}
