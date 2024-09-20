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
    schema::{articles, favorites},
    utils::converter,
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
    pub fn create(conn: &mut PgConnection, record: &CreateArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)?;
        Ok(article)
    }
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

    pub fn update(
        conn: &mut PgConnection,
        article_title_slug: &str,
        author_id: &Uuid,
        record: &UpdateArticle,
    ) -> Result<Self, AppError> {
        let t = articles::table
            .filter(Self::with_slug(article_title_slug))
            .filter(Self::with_author_id(author_id));
        let article = diesel::update(t).set(record).get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn fetch_by_slug_with_author(
        conn: &mut PgConnection,
        slug: &str,
    ) -> Result<(Self, User), AppError> {
        use crate::schema::users;

        let t = articles::table
            .inner_join(users::table)
            .filter(Self::with_slug(slug));
        let result = t.get_result::<(Self, User)>(conn)?;
        Ok(result)
    }

    pub fn convert_title_to_slug(title: &str) -> String {
        converter::to_kebab(title)
    }
}

impl Article {
    pub fn is_favorited_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        let t = favorites::table
            .select(diesel::dsl::count(favorites::id))
            .filter(Favorite::with_article_id(&self.id))
            .filter(Favorite::with_user_id(user_id));
        let count = t.first::<i64>(conn)?;
        Ok(count >= 1)
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

#[derive(Insertable, Clone)]
#[diesel(table_name = articles)]
pub struct CreateArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = articles)]
pub struct UpdateArticle {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
