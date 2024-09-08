use chrono::NaiveDateTime;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    dsl::Eq,
    prelude::*,
    PgConnection, QueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::features::{favorite::entities::Favorite, user::entities::User},
    error::AppError,
    schema::articles,
};

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

impl Article {
    fn with_author_id(author_id: &Uuid) -> WithAuthorId<&Uuid> {
        articles::author_id.eq(author_id)
    }

    fn with_slug(slug: &str) -> WithSlug<&str> {
        articles::slug.eq(slug)
    }

    fn with_id(id: &Uuid) -> WithId<&Uuid> {
        articles::id.eq(id)
    }
}

impl Article {
    pub fn fetch_ids_by_author_name(
        conn: &mut PgConnection,
        name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        use crate::schema::users;
        let t = users::table
            .inner_join(articles::table)
            .filter(User::with_username(name))
            .select(articles::id);
        let ids = t.load::<Uuid>(conn)?;
        Ok(ids)
    }

    pub fn fetch_favorites_count(&self, conn: &mut PgConnection) -> Result<i64, AppError> {
        use crate::schema::favorites;

        let t = favorites::table
            .filter(Favorite::with_article_id(&self.id))
            .select(diesel::dsl::count(favorites::created_at));
        let favorites_count = t.first::<i64>(conn)?;
        Ok(favorites_count)
    }
}
