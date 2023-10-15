use actix_web::{web, App, HttpServer};
mod routes;
mod models;
mod db;
mod controllers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // controllers::people::show_people().await;
  
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").to(|| async { "hello world" }))
			.service(routes::auth::user())
			.service(routes::auth::auth())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}