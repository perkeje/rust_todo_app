use actix_web::{ web, App, HttpServer};


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(todo_app::state::app::initialize_pool()))
        .configure(todo_app::config)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}