#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;

extern crate flexi_logger;
extern crate iron;
extern crate persistent;
extern crate rusqlite;


use hyper::server::Listening;

use iron::prelude::*;
use iron::error::HttpResult;

use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;

mod server;
mod db;

fn main() {
    setup_logging();
    let http_serv = make_http();
    println!("Hello, world!");
    {
        let conn = db::init_db_if_not_exist_and_connect("_SQLITE_DB");
        db::create_table_if_not_exists(&conn);
        let tag = "ATAG";
        let url = "some url";
        let referer = "some referer";
        let headers = "some headers";
        db::insert_log_entry(
            &conn,
            &tag, &url, &referer, &headers);
    }
    http_serv.unwrap();
}

fn make_http() -> HttpResult<Listening> {
    let any_addr = Ipv4Addr::from_str("0.0.0.0");
    let http_chain = Chain::new(server::tag_serve);
    
    return Iron::new(http_chain)
        .http((any_addr.unwrap(), 8181));
}

use flexi_logger::{LogConfig};
fn setup_logging() {
    let log_dir = setup_get_logging_dir();
    let log_config = log_config(log_dir);
    let log_init_res = flexi_logger::init(log_config, None);
    match log_init_res {
        Ok(_) => println!("Log initialized"),
        Err(error) => panic!("Issue starting the logger. {}", error)
    }
}

fn log_config(log_dir: String) -> LogConfig {
    let mut log_config = LogConfig::new();
    log_config.rotate_over_size = Some(1024 * 100_000);
    log_config.directory = Some(log_dir);
    log_config.log_to_file = true;
    log_config.timestamp = true;
    log_config.format = flexi_logger::detailed_format;
    log_config
}

use std::fs::DirBuilder;
fn setup_get_logging_dir() -> String {
    let log_dir = "./NEVER_COMMIT/flexi_logs";
    
    let path = PathBuf::from(log_dir);

    let mut dir_builder = DirBuilder::new();
    dir_builder.recursive(true);
    let was_created = dir_builder.create(path.as_path());
    match was_created {
        Ok(_) => (),
        Err(error) => println!("Directory not created, likely already exists. {}", error),
    }
    log_dir.to_owned()
}
