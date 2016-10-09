use iron::prelude::*;
use iron::error::HttpResult;
use iron::status;
use hyper::server::Listening;

use std::net::Ipv4Addr;
use std::str::FromStr;

mod router;

pub fn make_http() -> HttpResult<Listening> {
    let any_addr = Ipv4Addr::from_str("0.0.0.0");

    /*
    let http_chain = Chain::new(tag_serve);
    return Iron::new(http_chain)
        .http((any_addr.unwrap(), 8181));
    */

    let mut router = router::Router::new();
    router.init();
    
    return Iron::new(router).http((any_addr.unwrap(), 8181));
}
