use actix_web::{get, post, put, delete, web, Responder};
use actix_web_lab::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;

pub fn services() -> actix_web::Scope {
    web::scope("/lowongankerja")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Lowongankerja {
    id_lowongan: i16,
    deskripsi_lowongan: String,
    bidang_lowongan: String,
    keahlian_dibutuhkan: String,
}

#[get("/{id_lowongan}")]
async fn index(pool: web::Data<PgPool>, Path(id_lowongan): Path<i16>) -> Result<impl Responder> {
        let lowongankerja = sqlx::query_as::<_, Lowongankerja>(
        "SELECT id_lowongan, deskripsi_lowongan, bidang_lowongan, keahlian_dibutuhkan FROM lowongan_kerja"
    )
    .bind(id_lowongan)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(lowongankerja))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, Path(id_lowongan): Path<i16>,
    web::Json(lowongankerja): web::Json<Lowongankerja>,
) -> Result<impl Responder> {
    let _res = sqlx::query(
        "INSERT INTO lowongan_kerja (id_lowongan, deskripsi_lowongan, bidang_lowongan, keahlian_dibutuhkan) VALUES ($1, $2, $3, $4)",
    )
    .bind(id_lowongan)
    .bind(lowongankerja.deskripsi_lowongan)
    .bind(lowongankerja.bidang_lowongan)
    .bind(lowongankerja.keahlian_dibutuhkan)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diinput")
}

#[put("/update")]
async fn update(pool: web::Data<PgPool>, lowongankerja: web::Json<Lowongankerja>) -> Result<impl Responder> {
    let lowongankerja = lowongankerja.into_inner();
    let _res = sqlx::query(
        "UPDATE lowongan_kerja SET deskripsi_lowongan = $2, bidang_lowongan = $3, keahlian_dibutuhkan = $4 WHERE id_lowongan = $1",
    )
    .bind(lowongankerja.id_lowongan)
    .bind(lowongankerja.deskripsi_lowongan)
    .bind(lowongankerja.bidang_lowongan)
    .bind(lowongankerja.keahlian_dibutuhkan)
    
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diupdate")
}

#[delete("/{id_lowongan}")]
pub async fn delete(pool: web::Data<PgPool>, id_lowongan: web::Path<i16>,
    ) -> Result<impl Responder> {
    let id_lowongan = id_lowongan.into_inner();

    let _res = sqlx::query(
        "DELETE FROM lowongan_kerja WHERE id_lowongan = $1",
    )
    .bind(id_lowongan)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil dihapus")
}
