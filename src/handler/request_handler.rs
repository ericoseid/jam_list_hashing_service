use crate::RequestResponse;

pub trait RequestHandler {
  fn handle(&self, request_params: String, request_body: String) -> RequestResponse;
}