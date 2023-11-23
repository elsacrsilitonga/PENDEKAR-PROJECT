use actix_web::{get, post, put, delete, web, Responder};
use actix_web_lab::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    id_event: i16,
    bidang_event: String,
    poster_event: String,
    deskripsi_event: String,
}

pub fn services() -> actix_web::Scope {
    web::scope("/event")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/{id_event}")]
async fn index(pool: web::Data<PgPool>, Path(id_event): Path<i16>) -> Result<impl Responder> {
    let event = sqlx::query_as::<_, Event>(
        "SELECT id_event, bidang_event, poster_event, deskripsi_event FROM event WHERE id_event = $1",
    )
    .bind(id_event)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(event))
}

#[post("/")]
pub async fn create(pool: web::Data<PgPool>, event: web::Json<Event>) -> Result<impl Responder> {
    let event = event.into_inner();

    let _res = sqlx::query(
        "INSERT INTO event (bidang_event, poster_event, deskripsi_event) VALUES ($1, $2, $3)",
    )
        .bind(event.bidang_event)
        .bind(event.poster_event)
        .bind(event.deskripsi_event)
    
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diinput"))
}

#[put("/update")]
pub async fn update(pool: web::Data<PgPool>, event: web::Json<Event>) -> Result<impl Responder> {
    let event = event.into_inner();
    let _res = sqlx::query(
        "UPDATE event SET bidang_event = $2, poster_event = $3, deskripsi_event = $4 WHERE id_event = $1",
    )
    .bind(event.id_event)
    .bind(event.bidang_event)
    .bind(event.poster_event)
    .bind(event.deskripsi_event)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diupdate"))
}

#[delete("/{id_event}")]
pub async fn delete(pool: web::Data<PgPool>, id_event: web::Path<i16>) -> Result<impl Responder> {
    let id_event = id_event.into_inner();

    let _res = sqlx::query(
        "DELETE FROM event WHERE id_event = $1",
    )
    .bind(id_event)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil dihapus"))
}
