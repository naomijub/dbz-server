extern crate iron;

use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;

fn main() {
    Iron {
        handler: |_: &mut Request| {Ok(Response::with((status::Ok, "Ola! Eu sou o Goku!!!")))},
        threads: 8,
        timeouts: Timeouts {
            keep_alive: Some(Duration::from_secs(10)),
            read: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10))
    }}.http("localhost:3000").unwrap();
}
