use crate::{
    models::{task::Task, user::User},
    state::app::AppState,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};

pub async fn handle(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let user = match req.extensions_mut().remove::<User>() {
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish(),
    };

    let task_id = match req.match_info().get("id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().finish(),
    };

    match Task::change_done(task_id, &user.id, &state.get_connectinon()) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
