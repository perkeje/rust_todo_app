use actix_web::{web::{self},HttpRequest,HttpResponse, HttpMessage};
use serde::Deserialize;
use crate::models::user::User;
use crate::models::task::Task;
use crate::state::app::AppState;

#[derive(Deserialize)]
pub struct  Info{
     content: String
}

pub async fn handle(state:web::Data<AppState>, req:HttpRequest, new_content: web::Json<Info>) -> HttpResponse{
    let user = match req.extensions_mut().remove::<User>(){
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish() 
    };

    let task_id = match req.match_info().get("id"){
        Some(id) => id,
        None => return HttpResponse::BadRequest().finish() 
    };

    let content = new_content.into_inner().content;

    match Task::update_task(&task_id,&user.id,&state.get_connectinon(),&content){
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => return HttpResponse::BadRequest().finish() 
    }
}