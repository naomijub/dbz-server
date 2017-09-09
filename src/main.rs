extern crate iron;
extern crate iron_test;
extern crate router;

use iron::{Handler, status};
use iron::prelude::*;

use router::Router;

struct KiHandler;
struct NameHandler;

impl Handler for KiHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.extensions
            .get::<router::Router>()
            .expect("Router não encontrado a partir da extensão do Request.");
        let ki = params.find("ki").unwrap();

        Ok(Response::with((status::Ok, "Eh mais de ".to_string() + ki)))
    }
}

impl Handler for NameHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.extensions
            .get::<router::Router>()
            .expect("Router não encontrado a partir da extensão do Request.");
        let name = params.find("name").unwrap().to_owned();

        Ok(Response::with((status::Ok, name + "\nEu sou Goku!")))
    }
}

fn app_router() -> Router {
    let mut router = Router::new();
    router.get("valor/:ki", KiHandler, "ki router")
          .get("nome/:name", NameHandler, "name router");
    router
}

fn main() {
    Iron::new(app_router()).http("localhost:3000").unwrap();
}

#[cfg(test)]
mod test {
    use iron::Headers;

    use iron_test::{request, response};

    use super::{app_router};

    #[test]
    fn test_router_ki() {
        let response = request::get("http://localhost:3000/valor/8000",
                                    Headers::new(),
                                    &app_router());
        let result = response::extract_body_to_bytes(response.unwrap());

        assert_eq!(result, b"Eh mais de 8000");
    }

    #[test]
    fn test_router_name() {
        let response = request::get("http://localhost:3000/nome/Vegita",
                                    Headers::new(),
                                    &app_router());
        let result = response::extract_body_to_bytes(response.unwrap());

        assert_eq!(result, b"Vegita\nEu sou Goku!");
    }
}
