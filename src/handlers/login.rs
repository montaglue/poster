use actix_identity::Identity;
use actix_web::{post, Responder, HttpResponse, web, HttpRequest, HttpMessage};

use crate::{handlers::User, repositories::user::UserRepo};

#[post("/login")]
async fn login(req: HttpRequest, user: web::Json<User>, repo: web::Data<UserRepo>) -> impl Responder {
    let user = user.into_inner();

    if !User::check_nickname(&user.nickname) {
        return HttpResponse::BadRequest();
    }

    let model = log_error!(repo.get_user(user.nickname.clone()).await);

    if let Some(model) = model {
        if model.password != user.password {
            return HttpResponse::BadRequest();
        }
        log_error!(Identity::login(&req.extensions(), user.nickname));
        return HttpResponse::Ok();  
    }
    HttpResponse::BadRequest()
}