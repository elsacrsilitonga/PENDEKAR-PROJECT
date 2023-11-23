use actix_web::web;

pub mod register;
pub mod login;
pub mod profile;
pub mod event;
pub mod beasiswa;
pub mod lowongankerja;
// pub mod transaksi;
// pub mod notifikasi;

pub fn services() -> actix_web::Scope {
    web::scope("")
        .service( register      ::services() )
        .service( login         ::services() )
        .service( profile       ::services() )
        .service( event         ::services() )
        .service( beasiswa      ::services() )
        .service( lowongankerja ::services() )
        // .service( transaksi          ::services() )
        // .service( notifikasi ::services() )       
}