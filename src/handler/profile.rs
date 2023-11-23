use actix_web::{Responder, get, post, put, delete, web};
use actix_web_lab::extract::Path;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use crate::app::Result;

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
struct Profile {
    id_profile: i16,
    foto_profile: String,
    username: String,
    nama: String, 
    nomor_telepon: String,
    email: String,
    pendidikan_terakhir: PendidikanTerakhir,
    prodi: String,
    semester: Semester,
}

pub fn services() -> actix_web::Scope {
    web::scope("/profile")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/{id_profile}")]
async fn index(pool: web::Data<PgPool>, Path(id_profile): Path<i16>) -> Result<impl Responder> {

    let profile = sqlx::query_as::<_, Profile>(
        "SELECT id_profile, foto_profile, username, nama, nomor_telepon, email, pendidikan_terakhir, prodi, semester FROM profile WHERE id_profile = $1",
    )
    .bind(id_profile)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(profile))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, profile: web::Json<Profile>) -> Result<impl Responder> {
    let profile = profile.into_inner();
    let _res = sqlx::query(
        "INSERT INTO profile (foto_profile, username, nama, nomor_telepon, email, pendidikan_terakhir, prodi, semester) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(profile.foto_profile)
    .bind(profile.username)
    .bind(profile.nama)
    .bind(profile.nomor_telepon)
    .bind(profile.email)
    .bind(profile.pendidikan_terakhir)
    .bind(profile.prodi)
    .bind(profile.semester)

    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diinput"))
}

#[put("/updage")]
async fn update(pool: web::Data<PgPool>, profile: web::Json<Profile>) -> Result<impl Responder> {
    let profile = profile.into_inner();
    let _res = sqlx::query(
        "UPDATE profile SET foto_profile = $2, username = $3, nama = $4, nomor_telepon = $5, email = $6, pendidikan_terakhir = $7, prodi = $8, semester = $9 WHERE id_profile = $1",
    )
    .bind(profile.foto_profile)
    .bind(profile.username)
    .bind(profile.nama)
    .bind(profile.nomor_telepon)
    .bind(profile.email)
    .bind(profile.pendidikan_terakhir)
    .bind(profile.prodi)
    .bind(profile.semester)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil diupdate")
}

#[delete("/{id_profile}")]
pub async fn delete(pool: web::Data<PgPool>, id_profile: web::Path<i16>)  -> Result<impl Responder> {
    let id_profile = id_profile.into_inner();

    let _res = sqlx::query(
        "DELETE FROM profile WHERE id_profile = $1",
    )
    .bind(id_profile)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil dihapus"))
}
