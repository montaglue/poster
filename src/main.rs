#[macro_use]
pub mod utils;
mod repositories;
mod handlers;


use std::env;

use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web::{self}, App, HttpServer, cookie::Key, middleware};
use handlers::{register::register, login::login, post_message::post_message, load_messages::load_messages};
use repositories::{user::UserRepo, message::MessageRepo};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_uri = env::var("MONGOURI").unwrap();
    let secret_key = Key::generate();
    env_logger::init();

    let user_repo = UserRepo::new(mongo_uri.clone()).await;
    let message_repo = MessageRepo::new(mongo_uri.clone()).await;
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("poster-auth".to_owned())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(message_repo.clone()))
            .service(register)
            .service(login)
            .service(load_messages)
            .service(post_message)
            .service(Files::new("/", "./static/").index_file("index.html"))

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
