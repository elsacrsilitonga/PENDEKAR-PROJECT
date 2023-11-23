use actix_web::{get, post, put, delete, web, Responder};
use actix_web_lab::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::app::Result;

pub fn services() -> actix_web::Scope {
    web::scope("/markah")
        .service(index)
        .service(create)
        // .service(update)
        .service(delete)
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Markah {
    id_markah: i16,
    isi_markah: String,
}

#[get("/{id_markah}")]
async fn index(pool: web::Data<PgPool>, Path(id_markah): Path<i16>) -> Result<impl Responder> {
    let markah = sqlx::query_as::<_, Markah>(
        "select id_markah, isi_markah from markah",
    )
        .bind(id_markah)
        .fetch_one(pool.get_ref()) 
        .await? ;

    Ok( web::Json(markah) )
}

#[post("/")]
async fn create(pool: web::Data<PgPool>, markah: web::Json<Markah>) -> Result<impl Responder> {
    let markah = markah.into_inner();
    
    let _res = sqlx::query!(
        "insert into markah (isi_markah) values ($1)",
    )
        .bind(markah.isi_markah)
        .execute(pool.get_ref()) 
        .await? ;

    Ok("Data berhasil di input")
}

// #[put("/update")]
// async fn update(
//     pool: web::Data<PgPool>,
//     markah: web::Json<Markah>,
// ) -> Result<impl Responder> {
//     let markah = markah.into_inner();
//     let _res = sqlx::query!(
//         "UPDATE markah SET isi_markah = $2 WHERE id_markah = $1",
//         markah.id_markah,
//         .username,
//         login.password,
//     )
//     .execute(pool.get_ref())
//     .await?;

//     Ok("Data berhasil di update")
// }

#[delete("/{id_markah}")]
pub async fn delete(pool: web::Data<PgPool>, id_markah: web::Path<i16>) -> Result<impl Responder> {
    let id_markah = id_markah.into_inner();

    let _res = sqlx::query(
        "DELETE FROM markah WHERE id_markah = $1",
    )
    .bind(id_markah,)
    .execute(pool.get_ref())
    .await?;

    Ok("Data berhasil di hapus")
}