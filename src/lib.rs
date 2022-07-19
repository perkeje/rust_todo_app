#[macro_use]
extern crate diesel;

use actix_web::{ web, HttpResponse};


pub mod state;
pub mod routes;
pub mod models;
pub mod schema;
pub mod valid;
pub mod jwt;

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
        web::resource("/todos")
            .route(web::get().to(|| {HttpResponse::Ok()  }))
            .route(web::post().to(||HttpResponse::Ok()))
    );
    cfg.service(
        web::resource("/todos/{id}")
            .route(web::post().to(|| {HttpResponse::Ok()  }))
            .route(web::put().to(||HttpResponse::Ok()))
    );

}

