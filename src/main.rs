extern crate iron;
extern crate iron_test;
extern crate urlencoded;

use iron::{Handler, status};
use iron::prelude::*;

use urlencoded::UrlEncodedBody;

struct SpeechHandler;

impl Handler for SpeechHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let body = req.get_ref::<UrlEncodedBody>()
                      .expect("UrlEnconding is not correct");
        let name = body.get("name").unwrap()[0].to_owned();
        let action = body.get("action").unwrap()[0].to_owned();

        Ok(Response::with((status::Ok, name + ", " + &action + "!")))
    }
}

fn main() {
    Iron::new(SpeechHandler).http("localhost:3000").unwrap();
}

#[cfg(test)]
mod test {
    use iron::Headers;
    use iron::headers::ContentType;

    use iron_test::{request, response};

    use super::SpeechHandler;

    #[test]
    fn test_body() {
        let mut headers = Headers::new();
                headers.set(ContentType("application/x-www-form-urlencoded".parse().unwrap()));
        let response = request::post("http://localhost:3000/",
                                     headers,
                                     "name=Goku&action=Morra",
                                     &SpeechHandler);
        let result = response::extract_body_to_string(response.unwrap());

        assert_eq!(result, "Goku, Morra!");
    }
}
