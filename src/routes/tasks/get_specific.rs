use actix_web::{web::{self},HttpRequest,HttpResponse, HttpMessage};
use crate::models::user::User;
use crate::models::task::Task;
use crate::state::app::AppState;
pub async fn handle(state:web::Data<AppState>, req:HttpRequest) -> HttpResponse{
    let user = match req.extensions_mut().remove::<User>(){
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish() 
    };

    let task_id = match req.match_info().get("id"){
        Some(id) => id,
        None => return HttpResponse::BadRequest().finish() 
    };

    match Task::get_task_by_id(&task_id,&user.id,&state.get_connectinon()){
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => return HttpResponse::BadRequest().finish() 
    }
}