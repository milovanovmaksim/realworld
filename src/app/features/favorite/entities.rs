use chrono::NaiveDateTime;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::features::{article::entities::Article, user::entities::User},
    schema::favorites,
};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Clone)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updateed_at: NaiveDateTime,
}
