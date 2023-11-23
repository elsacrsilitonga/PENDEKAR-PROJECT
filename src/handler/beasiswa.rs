use actix_web::{get, post, put, delete, web, Responder};
use actix_web_lab::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;
use chrono::NaiveDateTime;

pub fn services() -> actix_web::Scope {
    web::scope("/beasiswa")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Beasiswa {
    id_beasiswa: i16,
    poster_beasiswa: String,
    nama_beasiswa: String,
    deskripsi_beasiswa: String,
    batas_pendaftaran: NaiveDateTime,
}

#[get("/{id_beasiswa}")]
async fn index(pool: web::Data<PgPool>, Path(id_beasiswa): Path<i16>) -> Result<impl Responder> {
    let beasiswa = sqlx::query_as::<_, Beasiswa>(
        "SELECT id_beasiswa, poster_beasiswa, nama_beasiswa, deskripsi_beasiswa, batas_pendaftaran FROM beasiswa"
    )
    .bind(id_beasiswa)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(beasiswa))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, beasiswa: web::Json<Beasiswa>) -> Result<impl Responder> {
    let beasiswa = beasiswa.into_inner();

    let _res = sqlx::query(
        "INSERT INTO beasiswa (poster_beasiswa, nama_beasiswa, deskripsi_beasiswa, batas_pendaftaran) VALUES ($1, $2, $3, $4)",
    )
    .bind(beasiswa.poster_beasiswa)
    .bind(beasiswa.nama_beasiswa)
    .bind(beasiswa.deskripsi_beasiswa) 
    .bind(beasiswa.batas_pendaftaran)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diinput"))
}

#[put("/update")]
pub async fn update(pool: web::Data<PgPool>, beasiswa: web::Json<Beasiswa>) -> Result<impl Responder> {
    let beasiswa = beasiswa.into_inner();
    let _res = sqlx::query(
        "UPDATE beasiswa SET poster_beasiswa = $2, nama_beasiswa = $3, deskripsi_beasiswa = $4, batas_pendaftaran = $5 WHERE id_beasiswa = $1",
    )
        .bind(beasiswa.id_beasiswa)
        .bind(beasiswa.poster_beasiswa)
        .bind(beasiswa.nama_beasiswa) 
        .bind(beasiswa.deskripsi_beasiswa)
        .bind(beasiswa.batas_pendaftaran)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diupdate"))
}

#[delete("/{id_beasiswa}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    id_beasiswa: web::Path<i16>,
) -> Result<impl Responder> {
    let id_beasiswa = id_beasiswa.into_inner();

    let _res = sqlx::query(
        "DELETE FROM beasiswa WHERE id_beasiswa = $1",
    )
    .bind(id_beasiswa)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil dihapus"))
}
