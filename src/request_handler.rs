extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate bodyparser;
extern crate persistent;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::typemap::Key;
use iron::middleware::Handler;
use persistent::State;

use models::*;

#[derive(Copy, Clone)]
pub struct NodePingState;
impl Key for NodePingState { type Value = i32; }

pub struct RequestHandler {
    pub redis_connection: String
}

impl RequestHandler {
    pub fn handle_index(request: &mut Request) -> IronResult<Response> {
        let mutex = request.get::<State<NodePingState>>().unwrap();
        let value = mutex.read().expect("mutex read");

        let metric = Metric::new(*value);
        let serialized = serde_json::to_string(&metric).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, serialized)))
    }

    pub fn handle_report(request: &mut Request) -> IronResult<Response> {
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
}

impl Handler for RequestHandler {
    pub fn handle(&self, request: &mut Request) -> IronResult<Response> {
        router!(
            get  "/"             => handle_index,
            post "/report/:key"  => handle_report
        )
        // match self.routes.get(&req.url.path.join("/")) {
        //     Some(handler) => handler.handle(req),
        //     None => Ok(Response::with(status::NotFound))
        // }
    }
}
