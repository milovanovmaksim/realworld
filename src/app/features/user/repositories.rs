use super::entities::{UpdateUser, User};
use crate::{
    app::features::{
        follow::entities::{CreateFollow, DeleteFollow, Follow},
        profile::entities::Profile,
    },
    error::AppError,
    utils::db::DbPool,
};
use diesel::prelude::*;
use uuid::Uuid;

type Token = String;

pub trait UserRepository: Send + Sync + 'static {
    fn signin(&self, email: &str, naive_password: &str) -> Result<(User, Token), AppError>;
    fn signup(
        &self,
        email: &str,
        user_name: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError>;

    fn update(&self, user_id: Uuid, changeset: UpdateUser) -> Result<(User, Token), AppError>;
    fn follow_user(&self, current_user: &User, target_username: &str) -> Result<Profile, AppError>;
    fn unfollow_user(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<Profile, AppError>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn signin(&self, email: &str, naive_password: &str) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        User::signin(conn, email, naive_password)
    }

    fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        User::signup(conn, email, username, naive_password)
    }
    fn update(&self, user_id: Uuid, changeset: UpdateUser) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        let new_user = User::update(conn, user_id, changeset)?;
        let token = &new_user.generate_token()?;
        Ok((new_user, token.clone()))
    }
    fn follow_user(&self, current_user: &User, target_username: &str) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let t = User::by_username(target_username);
        let followee = {
            use diesel::prelude::*;
            t.first::<User>(conn)?
        };
        Follow::create(
            conn,
            &CreateFollow {
                follower_id: current_user.id,
                followee_id: followee.id,
            },
        )?;
        Ok(Profile {
            username: current_user.username.clone(),
            bio: current_user.bio.clone(),
            image: current_user.image.clone(),
            following: true,
        })
    }

    fn unfollow_user(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let t = User::by_username(target_username);
        let followee = { t.first::<User>(conn)? };
        Follow::delete(
            conn,
            &DeleteFollow {
                followee_id: followee.id,
                follower_id: current_user.id,
            },
        )?;

        Ok(Profile {
            username: current_user.username.clone(),
            bio: current_user.bio.clone(),
            image: current_user.image.clone(),
            following: false,
        })
    }
}
