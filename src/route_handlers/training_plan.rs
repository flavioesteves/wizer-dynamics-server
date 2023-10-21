use actix_web::{web, web::ServiceConfig};

use crate::controllers::training_plan;

#[tracing::instrument(name = "Route Handler: Training Plan", skip(cfg))]
pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/training-plan")
            .service(
                web::resource("")
                    .route(web::get().to(training_plan::get_all_trainings))
                    .route(web::post().to(training_plan::add_training)),
            )
            .service(
                web::scope("/{training_id}").service(
                    web::resource("")
                        .route(web::get().to(training_plan::get_training_by_id))
                        .route(web::put().to(training_plan::update_training_by_id))
                        .route(web::delete().to(training_plan::delete_training_by_id)),
                ),
            ),
    );
}
