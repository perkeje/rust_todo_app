use crate::models::{task::Task};
use crate::models::user::User;
use crate::state::app::AppState;
use actix_web::{web::{self},HttpRequest,HttpResponse, HttpMessage};

#[derive(serde::Deserialize)]
pub struct PaginatedReq{
    page: Option<u32>,
    per_page: Option<u32>,
    checked: Option<bool>,
}

pub async fn handle(pagination: web::Json<PaginatedReq>,state: web::Data<AppState>, req:HttpRequest)->HttpResponse{
    let user = match req.extensions_mut().remove::<User>(){
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish()
    };

    let pagination = pagination.into_inner();

    let page = match pagination.page{
        Some(n) => n,
        None => 1
    };
    let per_page = match pagination.per_page{
        Some(n) => n,
        None => 1
    };
    let checked = match pagination.checked{
        Some(n) => n,
        None => false
    };

    match Task::get_tasks_paginated(page,per_page,&user.id,checked,&state.get_connectinon()){
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}