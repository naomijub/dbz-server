extern crate iron;
extern crate iron_test;

use iron::prelude::*;
use iron::{Handler, Headers, status};
#[allow(unused_imports)]
use iron_test::{request, response};

struct GokuHandler;

impl Handler for GokuHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Oi, eu sou o Goku!!!")))
    }
}

fn main() {
    Iron::new(GokuHandler).http("localhost:3000").unwrap();
}

#[test]
fn test_goku_handler() {
    let response = request::get("http://localhost:3000/hello",
                                Headers::new(),
                                &GokuHandler).unwrap();
    let result_body = response::extract_body_to_bytes(response);

    assert_eq!(result_body, b"Oi, eu sou o Goku!!!");
}
