use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscription {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<Subscription>) -> HttpResponse {
    println!("name is '{}' and email is '{}'", form.name, form.email);
    HttpResponse::Ok().finish()
}
