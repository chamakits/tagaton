use iron::headers as h;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::method::Method;
use iron::headers::*;
use iron::prelude::*;
use iron::error::HttpResult;
use iron::status;
use hyper::server::Listening;
use hyper::header;

use std::fmt;
use std::net::Ipv4Addr;
use std::io::Read;
use std::str::FromStr;
use router;
use time;
use unicase::UniCase;

use super::db;

pub fn make_http() -> HttpResult<Listening> {
    let any_addr = Ipv4Addr::from_str("0.0.0.0");

    let router = router!{
        id_1: get "/hello2" => hello_world,
        id_2: get "/do-nothing" => do_nothing,
        id_3: get "/tagg" => tagg_visit,
        id_4: get "/tagg/:given-tag" => tagg_visit,
        id_5: get "/img/:given-tag" => img_visit,
        id_6: post "/tagp/:given-tag" => tagp_visit,
        id_7: options "/tagp/:given-tag" => tagp_option,
        id_8: get format!("/taglist/all/{KEY}", KEY = "ABCDEF") => taglist_visit,
        id_9: get format!("/taglist/group/{KEY}", KEY = "ABCDEF") => taglist_group_visit,
    };
    return Iron::new(router).http((any_addr.unwrap(), 9292));
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
    //response.headers.set(AccessControlAllowOrigin::Any);
    default_visit(request, TagType::ImgGet, EMPTY_STRING).unwrap();
    Ok(response)
}

#[derive(Debug)]
pub enum TagType {
    TagPost,
    TagGet,
    ImgGet,
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct TagRequest {
    tag_type: TagType,
    tag: String,
    url: String,
    referer: String,
    headers: String,
    created_at: String,
    remote_addr: String,
}

// TODO: Consider separating this impl from retrieving so much from the request. Maybe a separate trait or something should be doing that.
impl TagRequest {
    /**
    TODO: Clean this up.
    Many issues with this codebase.  Specifically, needing to use format so much.
    Even a to_string.  I'm actually going to end up onlyi passing their references anyway
    That leads to tons of string assignments. Might not be necessary.
    **/
    fn new(request: &Request, tag_type: TagType) -> TagRequest {
        let given_tag = request.extensions.get::<router::Router>();
        let given_tag = given_tag.map(|params| {
            params.find("given-tag").unwrap_or("PARAM BUT NO TAG")
        });
        let referer = request.headers.get::<h::Referer>();
        let headers = &request.headers;

        let created_at = time::get_time();
        let created_at = time::at(created_at);

        TagRequest {
            tag_type: tag_type,
            tag: given_tag.unwrap_or_else(|| "Router extention missing").to_string(),
            url: format!("{}", request.url),
            referer: format!("{:?}", referer),
            headers: format!("{:?}", headers),
            created_at: format!("{}", created_at.rfc3339()),
            remote_addr: format!("{}", request.remote_addr),
        }
    }

    fn new_with_separate_referer( request: &mut Request, tag_type: TagType) -> TagRequest {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let referer_post: RefererPost = json::decode(&payload).unwrap();
        //TODO Be less wasteful, double calculating referer cause lazy.
        let mut tag_request = TagRequest::new(request, tag_type);
        tag_request.referer = referer_post.referer;
        tag_request
    }
}

fn tagg_visit(request: &mut Request) -> IronResult<Response> {
    default_visit(request, TagType::TagGet, "Tagg")
}

fn default_visit(
    request: &mut Request, tag_type: TagType,
    string_return:&'static str) -> IronResult<Response> {
    let tag_request = TagRequest::new(request, tag_type);
    insert_to_db(&tag_request);
    Ok(Response::with((status::Ok, string_return)))
}

fn insert_to_db(tag_request: &TagRequest) {
    (&DB_CONTROLLER).insert_log_entry(
        &tag_request.tag_type.to_string(),
        &tag_request.tag,
        &tag_request.url,
        &tag_request.referer,
        &tag_request.headers,
        &tag_request.created_at,
        &tag_request.remote_addr);
}

fn tags_grouped() -> Vec<db::GroupedTag> {
    (&DB_CONTROLLER).select_grouped_entries()
}

use rustc_serialize::json;

//TODO change to serde_json
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct RefererPost {
    referer: String,
}

fn setup_options(headers: &mut header::Headers) {
    headers.set(AccessControlAllowOrigin::Any);
    headers.set(AccessControlAllowHeaders(generate_control_allow_headers()));
    headers.set(AccessControlAllowMethods(generate_control_allow_methods()) );
    headers.set(AccessControlMaxAge(1728000u32));
    headers.set(AccessControlRequestHeaders(vec![UniCase("date".to_owned())]) );
    headers.set(AccessControlRequestMethod(Method::Post));

    fn generate_control_allow_methods() -> Vec<Method> {
        vec![
            Method::Get,
            Method::Post,
            Method::Patch,
            Method::Options]
    }

    fn generate_control_allow_headers() -> Vec<UniCase<String>> {
        vec![
            UniCase("X-Requested-With".to_owned()),
            UniCase("Content-Type".to_owned()),
            UniCase("Accept".to_owned()),
            UniCase("Origin".to_owned()),
        ]
    }
}

//TODO still not working. fix.
fn tagp_option(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, "TAGP"));
    setup_options(&mut (response.headers));
    println!("Request: {:?}", request);
    println!("Response: {:?}", response);
    Ok(response)
}

fn tagp_visit(request: &mut Request) -> IronResult<Response> {
    let tag_request = TagRequest::new_with_separate_referer(request, TagType::TagPost);
    println!("Tag request: {:?}", tag_request);
    insert_to_db(&tag_request);

    let mut response = Response::with((status::Ok, "TAGP"));
    setup_options(&mut (response.headers));
    Ok(response)
}

fn taglist_visit(request: &mut Request) -> IronResult<Response> {
    
    Ok(Response::with((status::Ok, "")))
}

fn taglist_group_visit(request: &mut Request) -> IronResult<Response> {
    let grouped_tags = tags_grouped();
    let payload = format!("{}", json::as_pretty_json(&grouped_tags));
    let mut response = Response::with((status::Ok, payload));
    response.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
    Ok(response)
}

fn do_nothing(_request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "did-nothing")))
}

fn hello_world(_request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World2 response !")))    
}
