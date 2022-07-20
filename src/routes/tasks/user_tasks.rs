use crate::models::task::Task;
use crate::models::user::User;
use crate::state::app::AppState;
use actix_web::{web::{self},HttpRequest,HttpResponse, HttpMessage};

pub async fn handle(state: web::Data<AppState>, req:HttpRequest)->HttpResponse{
    let user = match req.extensions_mut().remove::<User>(){
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish()
    };

    match Task::get_all_tasks(&user.id,&state.get_connectinon()){
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}