use actix_identity::Identity;
use actix_web::{post, Responder, HttpResponse, web};
use mongodb::bson::DateTime;

use crate::repositories::{user::UserRepo, message::{MessageRepo, MessageModel}};

use super::Message;

#[post("/post")]
pub async fn post_message(
    id: Identity, 
    message: web::Json<Message>, 
    user_repo: web::Data<UserRepo>, 
    message_repo: web::Data<MessageRepo>,
) -> impl Responder { // TODO: add proper error messages
    let message = message.into_inner();
    if let Ok(nickname) = id.id() {

        let user = match  user_repo.get_user(nickname).await {
            Ok(val) => val,
            Err(e) => {
                log::error!("Error: {:?}", e);
                panic!()
            }
        };


        if let Some(user) = user {
            let model = MessageModel {
                id: None,
                user_id: user.id.unwrap(),
                text: message.text,
                post_moment: DateTime::now(),
            };
            log_error!(message_repo.create_message(model).await);
            return HttpResponse::Ok();
        }
        return HttpResponse::BadRequest();
    } 
    HttpResponse::BadRequest()
}