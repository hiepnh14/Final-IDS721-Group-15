use actix_web::{web, HttpResponse, Responder};
use crate::state::{AppState, Message};

use crate::core::generator;

pub async fn post_message(prompt: web::Json<String>) -> impl Responder {
    match generator(prompt.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
pub async fn get_messages(state: web::Data<AppState>) -> impl Responder {
    let messages = state.messages.lock().unwrap();
    HttpResponse::Ok().json(&*messages)
}
