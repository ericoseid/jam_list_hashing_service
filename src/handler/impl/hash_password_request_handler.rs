extern crate json;


use crate::RequestHandler;
use crate::RequestResponse;
use crate::HashingHelper;

pub struct HashPasswordRequestHandler {
  pub hasher : Box<dyn HashingHelper>
}

impl RequestHandler for HashPasswordRequestHandler {
  fn handle(&mut self, _request_params: String, request_body: String) -> RequestResponse {
    let parse_result = json::parse(&request_body);

    let mut body_object = parse_result.unwrap();

    let hashed = self.hasher.hash_string(&body_object.remove("password").take_string().unwrap());

    RequestResponse::new("200", &hashed)
  }
}