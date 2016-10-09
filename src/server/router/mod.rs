use std::collections::HashMap;
use iron::{status, Handler};
use iron::prelude::*;
use super::super::db;

lazy_static! {
    static ref DB_CONTROLLER: db::DbController = db::DbController::new("_SQLIT_DB");
}

// Routing done by near exact copy of https://github.com/iron/iron/blob/master/examples/simple_routing.rs

pub struct Router {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }

    pub fn init(&mut self) {
        self.add_route("hello".to_string(), |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello World !")))
        });
        self.add_route("hello2".to_string(), hello_world);
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        
        match self.routes.get(&req.url.path().join("/")) {
            
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

use time;
fn hello_world(request: &mut Request) -> IronResult<Response> {
    let curr_time = time::now();
    let time_str = format!("{}",curr_time.rfc3339());
    {
        let db_conn = db::DbController::new("_SQLITE_DB");
        let tag = format!("ATAG at {}", time_str);
        let url = "some url";
        let referer = "some referer";
        let headers = "some headers";
        db_conn.insert_log_entry(
            &tag, &url, &referer, &headers);
    }
    Ok(Response::with((status::Ok, "Hello World2 !")))    
}
