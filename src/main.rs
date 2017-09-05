extern crate iron;
extern crate router;

use std::io::Read;
use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;
use router::Router;

fn main() {
    let mut router = Router::new();
        router.get("/", eu_sou_goku, "index")
            .get("/:query", ola_vc, "query")
            .post("/", retrucar, "post");

    Iron {
        handler: router,
        threads: 8,
        timeouts: Timeouts {
            keep_alive: Some(Duration::from_secs(10)),
            read: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10))
    }}.http("localhost:3000").unwrap();
}

fn eu_sou_goku(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Oi, eu sou o Goku!")))
}

fn retrucar(request: &mut Request) -> IronResult<Response>{
    let mut body = Vec::new();
    request
       .body
       .read_to_end(&mut body)
       .map_err(|e| IronError::new(e, (status::InternalServerError, "Error ao ler o request")))?;
    Ok(Response::with((status::Ok, body)))
}

fn ola_vc(request: &mut Request) -> IronResult<Response>{
    let ref query = request.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");

    Ok(Response::with((status::Ok, textinho(query))))
}

fn textinho(query: &str) -> String {
    let mut ola_string: String = "Olá ".to_owned();
    let eu_sou: &str = "\nEu sou Goku!";

    ola_string.push_str(query);
    ola_string.push_str(eu_sou);
    ola_string
}
