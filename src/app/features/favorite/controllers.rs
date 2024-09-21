use actix_web::{web, HttpRequest};

use crate::{
    app::drivers::middlewares::{auth, state::AppState},
    utils::api::ApiResponse,
};

type ArticleIdSlug = String;

pub async fn favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let article_title_slug = path.into_inner();
    state
        .di_container
        .favorite_usecase
        .favorite_article(current_user, article_title_slug)
}
