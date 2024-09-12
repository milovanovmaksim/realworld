use actix_web::web;
use serde::Deserialize;

use crate::{app::drivers::middlewares::state::AppState, utils::api::ApiResponse};

use super::usecases::FetchArticlesUsecaseInput;

#[derive(Deserialize)]
pub struct ArticlesListQueryParameter {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn index(
    state: web::Data<AppState>,
    params: web::Query<ArticlesListQueryParameter>,
) -> ApiResponse {
    let offset = std::cmp::min(params.offset.unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    state
        .di_container
        .article_usecase
        .fetch_articles(FetchArticlesUsecaseInput {
            tag: params.tag.clone(),
            author: params.author.clone(),
            favorited: params.favorited.clone(),
            offset,
            limit,
        })
}
