use actix_web::HttpResponse;

use super::entities::Profile as ProfileEntity;

pub struct ProfileResponse {
    pub profile: ProfileContent,
}

pub struct ProfileContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<ProfileEntity> for ProfileResponse {
    fn from(profile_entity: ProfileEntity) -> Self {
        let profile = ProfileContent {
            username: profile_entity.username,
            bio: profile_entity.bio,
            image: profile_entity.image,
            following: profile_entity.following,
        };
        ProfileResponse { profile }
    }
}

pub trait ProfilePresenter: Send + Sync + 'static {
    fn to_json(&self, entity: ProfileEntity) -> HttpResponse;
}

#[derive(Clone)]
pub struct ProfilePresenterImpl {}
impl ProfilePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProfilePresenter for ProfilePresenterImpl {
    fn to_json(&self, entity: ProfileEntity) -> HttpResponse {
        let response = ProfileResponse::from(entity);
        HttpResponse::Ok().json(response)
    }
}
