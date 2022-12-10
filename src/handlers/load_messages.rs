use actix_web::{post, Responder, HttpResponse, web};
use serde::{Serialize, Deserialize};

use crate::{repositories::message::{MessageRepo, MessageModel}, handlers::Message};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub number: u64,
}

impl Page {
    pub const SIZE: u64 = 5;
}

#[post("/page")]
pub async fn load_messages(page: web::Json<Page>, repo: web::Data<MessageRepo>) -> impl Responder {
    let page = page.into_inner();
    let messages = repo.get_messages((page.number - 1) * Page::SIZE, Page::SIZE).await;
    let messages: Vec<MessageModel> = log_error!(messages);
    let messages: Vec<Message> = messages
        .into_iter()
        .map(|msg| Message { text: msg.text })
        .collect();
    HttpResponse::Ok().json(messages)
}
