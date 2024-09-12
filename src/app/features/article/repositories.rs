use crate::{
    app::features::{
        favorite::entities::{Favorite, FavoriteInfo},
        profile::entities::Profile,
        tag::entities::Tag,
        user::entities::User,
    },
    error::AppError,
    schema::{articles, users},
    utils::db::DbPool,
};

use super::entities::Article;

pub trait ArticleRepository: Send + Sync + 'static {
    fn fetch_articles(
        &self,
        params: FetchArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError>;
}

pub struct FetchArticlesRepositoryInput {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Clone)]
pub struct ArticleRepositoryImpl {
    pool: DbPool,
}
impl ArticleRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
impl ArticleRepository for ArticleRepositoryImpl {
    fn fetch_articles(
        &self,
        params: FetchArticlesRepositoryInput,
    ) -> Result<(ArticlesList, ArticlesCount), AppError> {
        use crate::schema::{articles, tags, users};
        use diesel::prelude::*;

        let conn = &mut self.pool.get()?;
        let query = || {
            let mut query = articles::table.inner_join(users::table).into_boxed();

            if let Some(tag_name) = &params.tag {
                let ids = Tag::fetch_article_ids_by_name(conn, tag_name)?;
                query = query.filter(articles::id.eq_any(ids));
            }

            if let Some(author_name) = &params.author {
                let ids = Article::fetch_ids_by_author_name(conn, author_name)?;
                query = query.filter(articles::id.eq_any(ids))
            }

            if let Some(username) = &params.favorited {
                let ids = Favorite::fetch_favorited_artcile_ids_by_username(conn, username)?;
                query = query.filter(articles::id.eq_any(ids));
            }
            Ok(query)
        };
        let articles_count = query()?
            .select(diesel::dsl::count(articles::id))
            .first::<i64>(conn)?;
        let result = {
            let article_and_user_list = query()?
                .offset(params.offset)
                .limit(params.limit)
                .load::<(Article, User)>(conn)?;
            let tags_list = {
                let article_list = article_and_user_list
                    .clone()
                    .into_iter()
                    .map(|(article, _)| article)
                    .collect::<Vec<_>>();
                let tag_list = Tag::belonging_to(&article_list)
                    .order(tags::name.asc())
                    .load::<Tag>(conn)?;
                let tags_list: Vec<Vec<Tag>> = tag_list.grouped_by(&article_list);
                tags_list
            };
            let favorites_count_list = {
                let list: Result<Vec<_>, _> = article_and_user_list
                    .clone()
                    .into_iter()
                    .map(|(article, _)| article.fetch_favorites_count(conn))
                    .collect();
                list?
            };
            article_and_user_list
                .into_iter()
                .zip(favorites_count_list)
                .map(|((article, user), favorites_count)| {
                    (
                        article,
                        Profile {
                            username: user.username,
                            bio: user.bio,
                            image: user.image,
                            following: false, // NOTE: because not authz
                        },
                        FavoriteInfo {
                            is_favorited: false,
                            favorites_count,
                        },
                    )
                })
                .zip(tags_list)
                .collect::<Vec<_>>()
        };
        Ok((result, articles_count))
    }
}
type ArticlesListInner = (Article, Profile, FavoriteInfo);
pub type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;
type ArticlesCount = i64;
