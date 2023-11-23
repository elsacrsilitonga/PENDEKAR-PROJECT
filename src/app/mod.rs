use log::LevelFilter;
use chrono::Local;
use std::{io::Write, result};

pub mod error;
pub mod fetcher;
pub mod unwrapper;

pub use self::{fetcher::Fetcher, unwrapper::Unwrapper};

pub type Result<T> = result::Result<T, error::CustomError>;

// ini utk inisialisasi logger saja
pub fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .filter(Some("sqlx"), LevelFilter::Warn)
        .init();
}
