#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate flexi_logger;
extern crate iron;
extern crate persistent;
extern crate rusqlite;

mod server;
mod db;
mod my_log;

lazy_static! {
    static ref DB_CONTROLLER: db::DbController = db::DbController::new("_SQLIT_DB");
}

fn main() {
    my_log::setup_logging();
    let http_serv = server::make_http();
    println!("Hello, world!");
    {
        let db_conn = db::DbController::new("_SQLITE_DB");
        let tag = "ATAG";
        let url = "some url";
        let referer = "some referer";
        let headers = "some headers";
        db_conn.insert_log_entry(
            &tag, &url, &referer, &headers);
    }
    http_serv.unwrap();
}


