use actix_identity::Identity;
use actix_web::{post, Responder, HttpResponse, web, HttpRequest, HttpMessage};

use crate::{repositories::user::{UserModel, UserRepo}, handlers::User};

#[post("/register")]
async fn register(req: HttpRequest, user: web::Json<User>, repo: web::Data<UserRepo>) -> impl Responder {
    let user = user.into_inner();

    if !User::check_nickname(&user.nickname) {
        return HttpResponse::BadRequest();
    }

    let existed_user = log_error!(repo.get_user(user.nickname.clone()).await);
    if existed_user.is_some() {
        return HttpResponse::BadRequest();
    }

    let model = UserModel {
        id: None,
        nickname: user.nickname.clone(),
        password: user.password,
    };
    if let Err(e) = repo.create_user(model).await {
        log::error!("Error: {:?}", e);
        panic!();
    }
    log_error!(Identity::login(&req.extensions(), user.nickname));
    HttpResponse::Ok()
}


