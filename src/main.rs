#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;

extern crate flexi_logger;
extern crate iron;
extern crate persistent;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rustc_serialize;
extern crate time;
extern crate unicase;

mod server;
mod db;
mod my_log;

fn main() {
    my_log::setup_logging();
    let http_serv = server::make_http();
    println!("Hello, world!");
    
    http_serv.unwrap();
}
