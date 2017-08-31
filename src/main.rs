extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Ola Goku!!!")))
    }).http("localhost:3000").unwrap();
}
