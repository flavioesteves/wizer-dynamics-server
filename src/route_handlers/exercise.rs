use actix_web::{web, web::ServiceConfig};

use crate::controllers::exercise;

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/exercise")
            .service(
                web::resource("")
                    .route(web::get().to(exercise::get_all_exercises))
                    .route(web::post().to(exercise::add_exercise)),
            )
            .service(
                web::scope("/{exercise_id}").service(
                    web::resource("")
                        .route(web::get().to(exercise::get_exercise_by_id))
                        .route(web::put().to(exercise::update_exercise_by_id))
                        .route(web::delete().to(exercise::delete_exercise_by_id)),
                ),
            ),
    );
}
