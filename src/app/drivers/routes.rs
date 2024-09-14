use actix_web::web::{self, delete, get, post, put, ServiceConfig};

use crate::app;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/healthcheck")
                    .route("", get().to(app::features::healthcheck::controllers::index)),
            )
            .service(
                web::scope("/tags").route("", get().to(app::features::tag::controllers::index)),
            )
            .service(
                web::scope("/users")
                    .route(
                        "/login",
                        post().to(app::features::user::controllers::signin),
                    )
                    .route("", post().to(app::features::user::controllers::signup)),
            )
            .service(
                web::scope("/users")
                    .route("", get().to(app::features::user::controllers::me))
                    .route("", put().to(app::features::user::controllers::update)),
            )
            .service(
                web::scope("/profiles")
                    .route(
                        "/{username}",
                        get().to(app::features::profile::controllers::show),
                    )
                    .route(
                        "/{username}/follow",
                        post().to(app::features::profile::controllers::follow),
                    )
                    .route(
                        "/{username/follow}",
                        delete().to(app::features::profile::controllers::unfollow),
                    ),
            )
            .service(
                web::scope("/articles")
                    .route("/feed", get().to(app::features::article::controllers::feed))
                    .route("", get().to(app::features::article::controllers::index)),
            ),
    );
}
