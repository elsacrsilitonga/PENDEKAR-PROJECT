use actix_web::{get, post, put, delete, web, Responder};
use actix_web_lab::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;
use chrono::NaiveDateTime;

pub fn services() -> actix_web::Scope {
    web::scope("/transaksi")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaksi {
    id_transaksi: i16,
    bidang: String,
    tanggal_daftar: NaiveDateTime,
    tanggal_berakhir: NaiveDateTime,
}

#[get("/{id_transaksi}")]
async fn index(pool: web::Data<PgPool>, Path(id_transaksi): Path<i16>) -> Result<impl Responder> {
    let transaksi = sqlx::query_as::<_, Transaksi>(
        "SELECT id_transaksi, bidang, tanggal_daftar, tanggal_berakhir FROM transaksi"
    )
    .bind(id_transaksi)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(transaksi))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, transaksi: web::Json<Transaksi>) -> Result<impl Responder> {
    let transaksi = transaksi.into_inner();

    let _res = sqlx::query!(
        "INSERT INTO transaksi (bidang, tanggal_daftar, tanggal_berakhir) VALUES ($1, $2, $3)",
    )
    .bind(transaksi.bidang)
    .bind(transaksi.tanggal_daftar)
    .bind(transaksi.tanggal_berakhir)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diinput")
}

#[put("/update")]
async fn update(pool: web::Data<PgPool>, transaksi: web::Json<Transaksi>) -> Result<impl Responder> {
    let transaksi = transaksi.into_inner();

    let _res = sqlx::query!(
        "UPDATE transaksi SET bidang = $2, tanggal_daftar = $3, tanggal_berakhir = $4 WHERE id_transaksi = $1",
    )
    .bind(transaksi.id_transaksi)
    .bind(transaksi.bidang)
    .bind(transaksi.tanggal_daftar)
    .bind(transaksi.tanggal_berakhir)
    
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diupdate")
}

#[delete("/{id_transaksi}")]
pub async fn delete(pool: web::Data<PgPool>, id_transaksi: web::Path<i16>) -> Result<impl Responder> {
    let id_transaksi = id_transaksi.into_inner();

    let _res = sqlx::query(
        "DELETE FROM notifikasi WHERE id_transaksi = $1",
    )
    .bind(id_transaksi)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil dihapus")
}
