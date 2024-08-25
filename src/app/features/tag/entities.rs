use chrono::NaiveDateTime;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::{Queryable, QueryableByName},
    PgConnection, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::features::article::entities::Article, error::AppError, schema::tags};

#[derive(
    Identifiable,
    Selectable,
    Queryable,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Associations,
    QueryableByName,
)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {
    pub fn fetch(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = tags::table.load::<Self>(conn)?;
        Ok(list)
    }
}
