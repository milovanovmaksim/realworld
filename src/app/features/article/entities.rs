use chrono::NaiveDateTime;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    dsl::Eq,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::features::user::entities::User, schema::articles};

type WithAuthorId<T> = Eq<articles::author_id, T>;
type WithSlug<T> = Eq<articles::slug, T>;
type WithId<T> = Eq<articles::id, T>;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
