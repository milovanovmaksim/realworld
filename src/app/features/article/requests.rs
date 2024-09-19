use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub artcile: CreateArticleInner,
}

#[derive(Deserialize, Serialize)]
pub struct CreateArticleInner {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}
