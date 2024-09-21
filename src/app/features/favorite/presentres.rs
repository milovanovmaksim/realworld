use actix_web::HttpResponse;

use crate::app::features::{
    article::{entities::Article, presenters::SingleArticleResponse},
    profile::entities::Profile,
    tag::entities::Tag,
};

use super::entities::FavoriteInfo;

pub trait FavoritePresenter: Send + Sync + 'static {
    fn to_single_json(&self, item: (Article, Profile, FavoriteInfo, Vec<Tag>)) -> HttpResponse;
}

#[derive(Clone)]
pub struct FavoritePresenterImpl {}

impl FavoritePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl FavoritePresenter for FavoritePresenterImpl {
    fn to_single_json(
        &self,
        (article, profile, favorite_info, tags): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> HttpResponse {
        let res_model = SingleArticleResponse::from((article, profile, favorite_info, tags));
        HttpResponse::Ok().json(res_model)
    }
}
