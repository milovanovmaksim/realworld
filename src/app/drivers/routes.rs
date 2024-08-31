use actix_web::web::{self, get, ServiceConfig};

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
            ),
    );
}
