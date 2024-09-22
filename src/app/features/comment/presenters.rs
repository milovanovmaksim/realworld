use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::features::profile::entities::Profile, utils::date::Iso8601};

use super::entities::Comment;

#[derive(Serialize, Deserialize)]
pub struct SingleCommentResponse {
    pub comment: InnerComment,
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
}
