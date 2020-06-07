use crate::RequestResponse;

pub trait RequestHandler {
  fn handle(&mut self, request_params: String, request_body: String) -> RequestResponse;
}