use iron::headers as h;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::headers::ContentType;
use iron::prelude::*;
use iron::error::HttpResult;
use iron::status;
use hyper::server::Listening;

use std::net::Ipv4Addr;
use std::io::Read;
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
        id_4: get "/tagg/:given-tag" => tagg_visit,
        id_5: get "/img/:given-tag" => img_visit,
        id_6: post "/tagp" => tagp_visit,
    };
    return Iron::new(router).http((any_addr.unwrap(), 8181));
}

lazy_static! {
    static ref DB_CONTROLLER: db::DbController = {
        let mut dbc = db::DbController::new("_SQLITE_DB");
        dbc
    };
}

const EMPTY_STRING: &'static str = "";

fn img_visit(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, EMPTY_STRING ));
    //TODO this empty vector gets re-instantiated every time. Fix it.
    response.headers.set(ContentType(Mime(TopLevel::Image, SubLevel::Png, vec![])));
    tagg_visit(request).unwrap();
    Ok(response)
}

#[derive(Debug)]
struct TagRequest {
    tag: String,
    url: String,
    referer: String,
    headers: String,
}

// TODO: Consider separating this impl from retrieving so much from the request. Maybe a separate trait or something should be doing that.
impl TagRequest {
    /**
    TODO: Clean this up.
    Many issues with this codebase.  Specifically, needing to use format so much.
    Even a to_string.  I'm actually going to end up onlyi passing their references anyway
    That leads to tons of string assignments. Might not be necessary.
    **/
    fn new(request: &Request) -> TagRequest {
        let given_tag = request.extensions.get::<router::Router>();
        let given_tag = given_tag.map(|params| {
            params.find("given-tag").unwrap_or("PARAM BUT NO TAG")
        });
        let referer = request.headers.get::<h::Referer>();
        let headers = &request.headers;
        TagRequest {
            tag: given_tag.unwrap_or_else(|| "Router extention missing").to_string(),
            url: format!("{}", request.url),
            referer: format!("{:?}", referer),
            headers: format!("{:?}", headers),
        }
    }

    fn new_with_separate_referer( request: &mut Request) -> TagRequest {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let referer_post: RefererPost = json::decode(&payload).unwrap();
        //TODO Be less wasteful, double calculating referer cause lazy.
        let mut tag_request = TagRequest::new(request);
        tag_request.referer = referer_post.referer;
        tag_request
    }
}

fn tagg_visit(request: &mut Request) -> IronResult<Response> {
    let tag_request = TagRequest::new(request);
    /*
    insert_to_db (
        &tag_request.tag, &tag_request.url,
        &tag_request.referer, &tag_request.headers);
     */
    insert_to_db(&tag_request);
    
    Ok(Response::with((status::Ok, "Tagg")))    
}

fn insert_to_db(tag_request: &TagRequest) {
    (&DB_CONTROLLER).insert_log_entry(
        &tag_request.tag, &tag_request.url,
        &tag_request.referer, &tag_request.headers);
}

use rustc_serialize::json;

//TODO change to serde_json
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct RefererPost {
    referer: String,
}

fn tagp_visit(request: &mut Request) -> IronResult<Response> {
    /*
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let referer_post: RefererPost = json::decode(&payload).unwrap();
    println!("Referer found: {:?}", referer_post);
     */
    let tag_request = TagRequest::new_with_separate_referer(request);
        println!("Tag request: {:?}", tag_request);
    insert_to_db(&tag_request);
    Ok(Response::with((status::Ok, "TAGP")))
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
