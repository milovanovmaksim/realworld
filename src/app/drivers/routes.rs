use actix_web::web::{self, get, post, ServiceConfig};

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
            ),
    );
}
