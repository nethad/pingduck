extern crate iron;
extern crate duck;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate router;
extern crate bodyparser;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use persistent::State;
use iron::typemap::Key;
use duck::*;

#[derive(Copy, Clone)]
pub struct NodePingState;
impl Key for NodePingState { type Value = i32; }

fn app_router() -> Router {
    router!(
        get  "/"             => handle_index,
        post "/report/:key"  => handle_report
    )
}

fn handle_index(request: &mut Request) -> IronResult<Response> {
    let mutex = request.get::<State<NodePingState>>().unwrap();
    let value = mutex.read().expect("mutex read");

    let metric = Metric::new(*value);
    let serialized = serde_json::to_string(&metric).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, serialized)))
}

fn handle_report(request: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let struct_body = request.get::<bodyparser::Struct<StatusPing>>();
    match struct_body {
        Ok(Some(struct_body)) => {
            let mutex = request.get::<State<NodePingState>>().expect("get state");
            let mut value = mutex.write().expect("mutex write");
            *value = struct_body.value;

            let serialized = serde_json::to_string(&struct_body).unwrap();
            Ok(Response::with((content_type, status::Ok, serialized)))
        },
        _ => {
            Ok(Response::with((content_type, status::BadRequest, "{\"error\": \"bad request\"}")))
        }
    }
}

fn main() {
    println!("Running...");

    let mut chain = Chain::new(app_router());
    chain.link_before(State::<NodePingState>::one(0));

    Iron::new(chain).http("localhost:3000").unwrap();
}
