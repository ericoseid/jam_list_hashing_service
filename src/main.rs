#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate json;

#[path = "./data/request_response.rs"]
mod request_response;
#[path = "./handler/request_handler.rs"]
mod request_handler;
#[path = "./injection/request_handler_dependencies.rs"]
mod request_handler_dependencies;
#[path = "./hashing/hashing_helper.rs"]
mod hashing_helper;

use request_response::RequestResponse;
use request_handler::RequestHandler;
use hashing_helper::HashingHelper;
use std::io::Read;
use rocket::data::Data;
use rocket::data::DataStream;

#[post("/hash/password", data = "<data>")]
fn hash_password(data: Data) -> String {
  let mut handler = request_handler_dependencies::get_hash_password_request_handler();

  let mut body: String = String::from("");

  let mut data_stream: DataStream = data.open();
  data_stream.read_to_string(&mut body);

  let response = handler.handle(String::from(""), body);

  let ob = object!{
    status : &response.get_status_code()[..],
    body : &response.get_body()[..],
  };

  ob.dump()
}

pub fn main() {
  rocket::ignite().mount("/", routes![hash_password]).launch();
}
