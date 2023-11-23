use actix_web::{Responder, get, post, put, delete, web};
use actix_web_lab::extract::Path;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use crate::app::Result;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Login {
    id_login: i16,
    username: String,
    password: String,
}

pub fn services() -> actix_web::Scope {
    web::scope("/login")
        .service(index)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/{id_login}")]
async fn index(pool: web::Data<PgPool>, Path(id_login): Path<i16>) -> Result<impl Responder> {
    let login = sqlx::query_as::<_, Login>(
        "SELECT id_login, username, password FROM login WHERE id_login = $1",
    )
    .bind(id_login)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(web::Json(login))
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, login: web::Json<Login>) -> Result<impl Responder> {
    let login = login.into_inner();
    let _res = sqlx::query(
        "INSERT INTO login (username, password) VALUES ($1, $2)",
    )
    .bind(login.username)
    .bind(login.password)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diinput"))
}

#[put("/update")]
async fn update(pool: web::Data<PgPool>, login: web::Json<Login>) -> Result<impl Responder> {
    let login = login.into_inner();
    let _res = sqlx::query(
        "UPDATE login SET username = $2, password = $3 WHERE id_login = $1",
    )
    .bind(login.id_login)
    .bind(login.username)
    .bind(login.password)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil diupdate"))
}

#[delete("/{id_login}")]
pub async fn delete(pool: web::Data<PgPool>, id_login: web::Path<i16>) -> Result<impl Responder> {
    let id_login = id_login.into_inner();

    let _res = sqlx::query(
        "DELETE FROM login WHERE id_login = $1",
    )
    .bind(id_login)
    .execute(pool.get_ref())
    .await?;

    Ok(web::Json("Data berhasil dihapus"))
}
