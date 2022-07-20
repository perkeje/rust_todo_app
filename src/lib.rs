#[macro_use]
extern crate diesel;

use actix_web::{ web, HttpResponse};


pub mod state;
pub mod routes;
pub mod models;
pub mod schema;
pub mod valid;
pub mod jwt;
pub mod middleware;

pub fn config(cfg: &mut web::ServiceConfig){

    cfg.service(
        web::resource("/").route(web::get().to(|| {HttpResponse::Ok()  }))
    );

    cfg.service(
        web::resource("/login").route(web::post().to(routes::auth::login::handle))
    );
    cfg.service(
        web::resource("/register").route(web::post().to(routes::auth::register::handle))
    );
    cfg.service(
        web::resource("/tasks")
            .route(web::post().to(routes::tasks::new::handle))
            .route(web::get().to(routes::tasks::user_tasks::handle))
            .route(web::delete().to(routes::tasks::delete_all::handle))
            .wrap(crate::middleware::auth::AuthGuard)
    );
    cfg.service(
        web::resource("/tasks/{id}")
            .route(web::get().to(routes::tasks::get_specific::handle))
            .route(web::put().to(||HttpResponse::Ok()))
            .route(web::delete().to(routes::tasks::delete_specific::handle))
            .wrap(crate::middleware::auth::AuthGuard)
    );

}

