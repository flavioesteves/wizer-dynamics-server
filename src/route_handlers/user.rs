use actix_web::{web, web::ServiceConfig};

use crate::controllers::user;

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/user/get_all_users")
            .service(web::resource("").route(web::get().to(user::get_all_users))),
    )
    .service(
        web::scope("/user/logged_user")
            .service(web::resource("").route(web::get().to(user::get_user_logged))),
    )
    .service(
        web::scope("/user/{user_id}")
            .service(web::resource("").route(web::get().to(user::get_user_by_id))),
    )
    .service(
        web::scope("/user/test").service(web::resource("").route(web::post().to(user::post_user))),
    );
}
