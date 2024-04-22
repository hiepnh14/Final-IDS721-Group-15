use actix_web::{web, HttpResponse, Responder};
use crate::state::{AppState, Message};

use crate::core::generate_response;

pub async fn post_message(prompt: web::Json<String>) -> impl Responder {
    match generate_response(prompt.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
pub async fn get_messages(state: web::Data<AppState>) -> impl Responder {
    let messages = state.messages.lock().unwrap();
    HttpResponse::Ok().json(&*messages)
}
