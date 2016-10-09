use std::collections::HashMap;
use iron::{status, Handler};
use iron::prelude::*;

// Near exact copy of https://github.com/iron/iron/blob/master/examples/simple_routing.rs

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
