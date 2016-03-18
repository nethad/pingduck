#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate serde;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate router;

// use iron::prelude::*;
// use iron::status;
// use iron::mime::Mime;

pub mod models;
pub mod request_handler;
