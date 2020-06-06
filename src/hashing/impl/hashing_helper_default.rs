extern crate crypto;

use crate::HashingHelper;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

pub struct HashingHelperDefault {}

impl HashingHelper for HashingHelperDefault {
  fn hash_string(&self, s: &String) -> String {
    let mut hasher = Sha512::new();

    hasher.input_str(s);

    hasher.result_str()
  }
}