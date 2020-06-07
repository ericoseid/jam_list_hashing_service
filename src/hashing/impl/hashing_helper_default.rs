extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crate::HashingHelper;

pub struct HashingHelperDefault {
  pub hasher: Sha512
}

impl HashingHelper for HashingHelperDefault {
  fn hash_string(&mut self, s: &String) -> String {
    self.hasher.input_str(s);

    self.hasher.result_str()
  }
}