use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::features::profile::entities::Profile, utils::date::Iso8601};

use super::entities::Comment;

#[derive(Serialize, Deserialize)]
pub struct SingleCommentResponse {
    pub comment: InnerComment,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleCommentResponse {
    pub comments: Vec<InnerComment>,
}

impl From<(Comment, Profile)> for SingleCommentResponse {
    fn from((comment, profile): (Comment, Profile)) -> Self {
        Self {
            comment: InnerComment {
                id: comment.id,
                created_at: Iso8601(comment.created_at),
                updated_at: Iso8601(comment.updated_at),
                body: comment.body,
                author: InnerAuthor {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
            },
        }
    }
}

impl From<Vec<(Comment, Profile)>> for MultipleCommentResponse {
    fn from(list: Vec<(Comment, Profile)>) -> Self {
        Self {
            comments: list
                .into_iter()
                .map(|item| {
                    let (comment, profile) = item;
                    InnerComment {
                        id: comment.id,
                        created_at: Iso8601(comment.created_at),
                        updated_at: Iso8601(comment.updated_at),
                        body: comment.body,
                        author: InnerAuthor {
                            username: profile.username,
                            bio: profile.bio,
                            image: profile.image,
                            following: profile.following,
                        },
                    }
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InnerComment {
    pub id: Uuid,
    pub created_at: Iso8601,
    pub updated_at: Iso8601,
    pub body: String,
    pub author: InnerAuthor,
}

#[derive(Serialize, Deserialize)]
pub struct InnerAuthor {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

pub trait CommentPresenter: Send + Sync + 'static {
    fn to_single_json(&self, item: (Comment, Profile)) -> HttpResponse;
    fn to_multi_json(&self, list: Vec<(Comment, Profile)>) -> HttpResponse;
    fn to_http_res(&self) -> HttpResponse;
}

#[derive(Clone)]
pub struct CommentPresenterImpl {}

impl CommentPresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommentPresenter for CommentPresenterImpl {
    fn to_single_json(&self, item: (Comment, Profile)) -> HttpResponse {
        let res = SingleCommentResponse::from(item);
        HttpResponse::Ok().json(res)
    }

    fn to_multi_json(&self, list: Vec<(Comment, Profile)>) -> HttpResponse {
        let res = MultipleCommentResponse::from(list);
        HttpResponse::Ok().json(res)
    }

    fn to_http_res(&self) -> HttpResponse {
        HttpResponse::Ok().json("OK")
    }
}
