extern crate json;

#[path = "../../hashing/hashing_helper.rs"]
mod hashing_helper;

use crate::RequestHandler;
use crate::RequestResponse;
use hashing_helper::HashingHelper;

pub struct HashPasswordRequestHandler {
  pub hasher : HashingHelper
}

impl RequestHandler for HashPasswordRequestHandler {
  fn handle(&self, request_params: String, request_body: String) -> RequestResponse {
    let parseResult = json::parse(request_body);

    let bodyObject = parseResult.unwrap();

    self.hasher.hash_string(bodyObject.remove("password").take_string().unwrap())
  }
}