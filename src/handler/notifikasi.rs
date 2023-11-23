use actix_web::{get, post, put, delete, web, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;
use chrono::NaiveDateTime;

pub fn services() -> actix_web::Scope {
    web::scope("/notifikasi")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize)]
pub struct Notifikasi {
    id_notifikasi: i16,
    jenis_notifikasi: String,
    waktu_notifikasi: NaiveDateTime,
}

#[get("/{id_notifikasi}")]
pub async fn index(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let notifikasi = sqlx::query_as!(Notifikasi,
        "SELECT id_notifikasi, jenis_notifikasi, waktu_notifikasi, FROM notifikasi"
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(notifikasi))
}

#[post("/")]
pub async fn create(pool: web::Data<PgPool>, notifikasi: web::Json<Event>) -> Result<impl Responder> {
    let notifikasi = notifikasi.into_inner();

    let _res = sqlx::query!(
        "INSERT INTO notifikasi (jenis_notifikasi, waktu_notifikasi) VALUES ($1, $2)",
        notifikasi.jenis_notifikasi, notifikasi.waktu_notifikasi
    )
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diinput")
}

#[put("/update")]
pub async fn update(pool: web::Data<PgPool>, notifikasi: web::Json<Notifikasi>) -> Result<impl Responder> {
    let notifikasi = notifikasi.into_inner();

    let _res = sqlx::query!(
        "UPDATE notifikasi SET jenis_notifikasi = $2, waktu_notifikasi = $3 WHERE id_notikasi = $1",
        notifikasi.id_notifikasi,
        notifikasi.jenis_notifikasi, 
        notifikasi.waktu_notifikasi 
    )
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diupdate")
}

#[delete("/{id_notifikasi}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    id_notifikasi: web::Path<i16>,
) -> Result<impl Responder> {
    let id_notifikasi = id_notifikasi.into_inner();

    let _res = sqlx::query!(
        "DELETE FROM notifikasi WHERE id_notifikasi = $1",
        id_notifikasi,
    )
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil dihapus")
}
