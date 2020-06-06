#[path = "./data/request_response.rs"]
mod request_response;
#[path = "./handler/request_handler.rs"]
mod request_handler;

use request_response::RequestResponse;
use request_handler::RequestHandler;

pub fn main() {
  let r = request_response::RequestResponse::new("Yeeet", "yeeet");

  println!("{}", r.get_status_code());
  println!("{}", r.get_status_code());

  println!("{}", r.get_body());
  println!("{}", r.get_body());
}
