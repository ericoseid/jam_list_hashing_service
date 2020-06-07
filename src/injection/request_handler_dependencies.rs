extern crate crypto;

#[path = "../handler/impl/hash_password_request_handler.rs"]
mod hash_password_request_handler;
#[path = "../hashing/impl/hashing_helper_default.rs"]
mod hashing_helper_default_impl;

use crate::RequestHandler;
use hash_password_request_handler::HashPasswordRequestHandler;
use hashing_helper_default_impl::HashingHelperDefault;
use crypto::sha2::Sha512;
use crate::HashingHelper;

pub fn get_hash_password_request_handler() -> Box<dyn RequestHandler> {
  Box::new(HashPasswordRequestHandler {
    hasher : get_hashing_helper_default()
  })
}

pub fn get_hashing_helper_default() -> Box<dyn HashingHelper> {
  Box::new(HashingHelperDefault {
    hasher : get_sha_512()
  })
}

pub fn get_sha_512() -> Sha512 {
  Sha512::new()
}