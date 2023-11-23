use actix_web::{Responder, get, post, put, delete, web};
// use actix_web_lab::extract::Path;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use crate::app::Result;
use chrono::NaiveDateTime; // Import NaiveDate

pub fn services() -> actix_web::Scope {
    web::scope("/register")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pendidikan_terakhir")]
enum PendidikanTerakhir { SD, SMP, SMA, S1, S2, S3 }

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "semester")]
enum Semester {
    Semester1, Semester2, Semester3, Semester4,
    Semester5, Semester6, Semester7, Semester8,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Register {
    id_register: i32,
    register_date: NaiveDateTime,
    nama: String,
    tanggal_lahir: NaiveDateTime, 
    nomor_telepon: String,
    email: String,
    username: String,
    password: String,
    pendidikan_terakhir: PendidikanTerakhir,
    prodi: String,
    semester: Semester,
}

#[get("/{id_register}")]
async fn index(pool: web::Data<PgPool>, path: web::Path<i16>) -> Result<impl Responder> {
    let register = sqlx::query_as::<_, Register>(
        "SELECT id_register, register_date, nama, tanggal_lahir, nomor_telepon, email, username, password, pendidikan_terakhir, prodi, semester FROM register WHERE id_register = $1",
    )
    .bind(path.into_inner())
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(register))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, register: web::Json<Register>) -> Result<impl Responder> {
    let register = register.into_inner();

    let _res = sqlx::query(
        "INSERT INTO register (register_date, nama, tanggal_lahir, nomor_telepon, email, username, password, pendidikan_terakhir, prodi, semester) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(&register.register_date)
        .bind(&register.nama)
        .bind(&register.tanggal_lahir)
        .bind(&register.nomor_telepon)
        .bind(&register.email)
        .bind(&register.username)
        .bind(&register.password)
        .bind(&register.pendidikan_terakhir)
        .bind(&register.prodi)
        .bind(&register.semester)
        .execute(pool.get_ref())
        .await?;

        Ok(web::Json("Data berhasil diinput"))
}

#[put("/update")]
async fn update(pool: web::Data<PgPool>, register: web::Json<Register>) -> Result<impl Responder> {
    let register = register.into_inner();

    let _res = sqlx::query(
        "UPDATE member SET register_date = $2, nama = $3, tanggal_lahir = $4, nomor_telepon = $5, email = $6, username = $7, password = $8, pendidikan_terakhir = $9, prodi = $10, semester = $11 WHERE id_register = $1")
        .bind(&register.id_register)
        .bind(&register.register_date)
        .bind(&register.nama)
        .bind(&register.tanggal_lahir)
        .bind(&register.nomor_telepon)
        .bind(&register.email)
        .bind(&register.username)
        .bind(&register.password)
        .bind(&register.pendidikan_terakhir)
        .bind(&register.prodi)
        .bind(&register.semester)
        .execute(pool.get_ref())
        .await?;

    Ok(web::Json("Data berhasil diupdate"))
}

#[delete("/{id_register}")]
pub async fn delete(pool: web::Data<PgPool>, id_register: web::Path<i32>, // Kembalikan tipe id menjadi i16 jika diperlukan
) -> Result<impl Responder> {
    let id_register = id_register.into_inner();

    let _res = sqlx::query("DELETE FROM register WHERE id_register = $1")
        .bind(id_register)
        .execute(pool.get_ref())
        .await?;

    Ok(web::Json("Data berhasil dihapus"))
}