use crate::models::task::NewTask;
use crate::models::user::User;
use crate::state::app::AppState;
use actix_web::{web::{self, Json},HttpRequest,HttpResponse, HttpMessage};

pub async fn handle(req: HttpRequest, data: Json<NewTask>, state:web::Data<AppState>)->HttpResponse{
    
    let user = match req.extensions_mut().remove::<User>(){
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish()
    };
    
    let content =  &data.content;

    match NewTask::create(&state.get_connectinon(),&content,&user.id){
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}