use iron::headers as h;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::headers::ContentType;
use iron::prelude::*;
use iron::error::HttpResult;
use iron::{status, Handler};
use hyper::server::Listening;

use std::net::Ipv4Addr;
use std::io;
use std::str::FromStr;
use router;
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
    let mut router = router::Router::new();
    router.get("/hello2", hello_world);
     */

    let router = router!{
        id_1: get "/hello2" => hello_world,
        id_2: get "/do-nothing" => do_nothing,
        id_3: get "/tagg" => tagg_visit,
        id_4: get "/tagg/:uniq-tag" => tagg_visit,
        id_5: get "/img/:uniq-tag" => img_visit,
    };
    return Iron::new(router).http((any_addr.unwrap(), 8181));
}

lazy_static! {
    static ref DB_CONTROLLER: db::DbController = {
        let mut dbc = db::DbController::new("_SQLITE_DB");
        dbc
    };
    static ref EMPTY_BYTES: Vec<u8> = {
        let mut v = Vec::new();
        v
    };
    static ref EMPTY_ARR: [u8;0] = [];
    static ref STR_EMPTY: &'static str = "";
}

fn img_visit(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, "" ));
    //TODO figure out how to get this to work
    //let mut response = Response::with((status::Ok, EMPTY_BYTES ));
    response.headers.set(ContentType(Mime(TopLevel::Image, SubLevel::Png, vec![])));
    tagg_visit(request);
    Ok(response)
}

fn tagg_visit(request: &mut Request) -> IronResult<Response> {
    let uniq_tag = request.extensions.get::<router::Router>();
    let uniq_tag = uniq_tag.map(|params| {
        params.find("uniq-tag").unwrap_or("PARAM BUT NO TAG")
    });

    let referer = request.headers.get::<h::Referer>();
    let headers = &request.headers;

    let db_conn = &DB_CONTROLLER;
    let tag = uniq_tag.unwrap_or_else(|| "Router extention missing");
    let url = format!("{}", request.url);
    let referer = format!("{:?}", referer);
    let headers = format!("{:?}", headers);
    db_conn.insert_log_entry(
        &tag, &url, &referer, &headers);

    Ok(Response::with((status::Ok, "Tagg")))    
}


fn do_nothing(_request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "did-nothing")))
}

fn hello_world(_request: &mut Request) -> IronResult<Response> {
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
