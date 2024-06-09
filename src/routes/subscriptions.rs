use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<Subscription>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "request_id {} - Adding {} with email, {} as new subscriber",
        request_id,
        form.name,
        form.email
    );
    tracing::info!(
        "request_id {} - Saving new subscriber info in the database",
        request_id
    );

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now().naive_utc()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber info saved successfully",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok().finish()
}
