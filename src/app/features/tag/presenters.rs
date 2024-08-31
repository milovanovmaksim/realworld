use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use super::entities::Tag;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TagResponse {
    pub tags: Vec<String>,
}

impl std::convert::From<Vec<Tag>> for TagResponse {
    fn from(tags: Vec<Tag>) -> Self {
        let list = tags.iter().map(|tag| tag.name.clone()).collect();
        TagResponse { tags: list }
    }
}

pub trait TagPresenter: Send + Sync + 'static {
    fn to_json(&self, list: Vec<Tag>) -> HttpResponse;
}

#[derive(Clone)]
pub struct TagPresenterImpl {}

impl TagPresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl TagPresenter for TagPresenterImpl {
    fn to_json(&self, list: Vec<Tag>) -> HttpResponse {
        let res = TagResponse::from(list);
        HttpResponse::Ok().json(res)
    }
}
