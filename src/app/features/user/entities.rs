use crate::utils::{hasher, token};
use crate::{error::AppError, schema::users};
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{
    associations::Identifiable,
    backend::Backend,
    deserialize::Queryable,
    dsl::{AsSelect, Eq, Filter, Select},
    ExpressionMethods, PgConnection, QueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Token = String;
type All<DB> = Select<users::table, AsSelect<User, DB>>;
type WithEmail<T> = Eq<users::email, T>;
type ByEmail<DB, T> = Filter<All<DB>, WithEmail<T>>;

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
}

impl User {
    fn all<DB>() -> All<DB>
    where
        DB: Backend,
    {
        users::table.select(User::as_select())
    }
    fn with_email(email: &str) -> WithEmail<&str> {
        users::email.eq(email)
    }

    fn by_email<DB>(email: &str) -> ByEmail<DB, &str>
    where
        DB: Backend,
    {
        Self::all().filter(Self::with_email(email))
    }
}

impl User {
    pub fn signin(
        conn: &mut PgConnection,
        email: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let t = Self::by_email(email).limit(1);
        let user: User = t.first(conn)?;
        hasher::verify(naive_password, &user.password)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn signup<'a>(
        conn: &mut PgConnection,
        email: &'a str,
        username: &'a str,
        naive_password: &'a str,
    ) -> Result<(User, Token), AppError> {
        let hashed_passowrd = hasher::hash_password(naive_password)?;
        let record = SignupUser {
            email,
            username,
            password: &hashed_passowrd,
        };
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;
        let token = user.generate_token()?;
        Ok((user, token))
    }
}

impl User {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000;
        let token = token::generate(self.id, now)?;
        Ok(token)
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
