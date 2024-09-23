use diesel::QueryDsl;

use crate::{
    app::features::{
        article::entities::{Article, FetchBySlugAndAuthorId},
        profile::entities::Profile,
        user::entities::User,
    },
    error::AppError,
    schema::{comments, users},
    utils::db::DbPool,
};

use super::entities::{Comment, CreateComment};

pub trait CommentRepository: Send + Sync + 'static {
    fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<(Comment, Profile), AppError>;
    fn fetch_comments(
        &self,
        current_user: &Oprion<User>,
    ) -> Result<Vec<(Commentm, Profile)>, AppError>;
}

#[derive(Clone)]
pub struct CommentRepositoryImpl {
    pool: DbPool,
}

impl CommentRepository for CommentRepositoryImpl {
    fn create_comment(
        &self,
        body: String,
        article_title_slug: String,
        author: User,
    ) -> Result<(Comment, Profile), AppError> {
        let conn = &mut self.pool.get()?;
        let article = Article::fetch_by_slug_and_author_id(
            conn,
            &FetchBySlugAndAuthorId {
                slug: article_title_slug,
                author_id: author.id,
            },
        )?;
        let comment = Comment::create(
            conn,
            &CreateComment {
                body,
                author_id: author.id,
                article_id: article.id,
            },
        )?;
        let profile = author.fetch_profile(conn, &author.id)?;
        Ok((comment, profile))
    }
    fn fetch_comments(
        &self,
        current_user: &Option<User>,
    ) -> Result<Vec<(Comment, Profile)>, AppError> {
        let conn = &mut self.pool.get()?;

        let comments = comments::table
            .inner_join(users::table)
            .get_results::<(Comment, User)>(conn)?;
        let comments = comments
            .iter()
            .map(|(comment, user)| {
                let profile = user.to_profile(conn, current_user);
                (comment, profile)
            })
            .collect::<Vec<(Comment, Profile)>>();
        Ok(comments)
    }
}
