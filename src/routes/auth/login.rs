use crate::models::authentification::AuthUser;
use crate::state::app::AppState;
use actix_web::{web, HttpResponse};

pub async fn handle(user:web::Json<AuthUser>, state: web::Data<AppState>)-> HttpResponse{
    match user.into_inner().authenticate(&state.get_connectinon()){
        Ok((user,token)) => HttpResponse::Ok().insert_header(("jwt",token)).json(user),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}