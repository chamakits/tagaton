use iron::prelude::*;
use iron::error::HttpResult;
use iron::{status, Handler};
use hyper::server::Listening;

use std::net::Ipv4Addr;
use std::str::FromStr;
use router as rust_router;
use time;

use super::db;

pub fn make_http() -> HttpResult<Listening> {
    let any_addr = Ipv4Addr::from_str("0.0.0.0");

    /*
    let mut router = router::Router::new();
    router.init();
    return Iron::new(router).http((any_addr.unwrap(), 8181));
     */

    /*
    let mut router = rust_router::Router::new();
    router.get("/hello2", hello_world);
     */

    let router = router!{
        id_1: get "/hello2" => hello_world,
        id_2: get "/do-nothing" => do_nothing,
    };
    return Iron::new(router).http((any_addr.unwrap(), 8181));
}

fn tagg_visit(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Tagg")))    
}

lazy_static! {
    static ref DB_CONTROLLER: db::DbController = {
        let mut dbc = db::DbController::new("_SQLITE_DB");
        dbc
    };
}

fn do_nothing(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "did-nothing")))
}

fn hello_world(request: &mut Request) -> IronResult<Response> {
    let curr_time = time::now();
    let time_str = format!("{}",curr_time.rfc3339());
    {
        //let db_conn = db::DbController::new("_SQLITE_DB");
        let db_conn = &DB_CONTROLLER;
        let tag = format!("ATAG at {}", time_str);
        let url = "some url";
        let referer = "some referer";
        let headers = "some headers";
        db_conn.insert_log_entry(
            &tag, &url, &referer, &headers);
    }
    Ok(Response::with((status::Ok, "Hello World2 response !")))    
}
