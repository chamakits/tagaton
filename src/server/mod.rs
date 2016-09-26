use iron::prelude::*;
use iron::status;

pub fn tag_serve(request: &mut Request) -> IronResult<Response> {
    //TODO continue here
    let _req = request;
    return Ok(Response::with((status::Ok, "Tag thingy")));
}
