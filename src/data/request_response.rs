pub struct RequestResponse {
  status_code : String,
  body : String
}

impl RequestResponse {
  pub fn get_status_code(&self) -> &String {
    &self.status_code
  }

  pub fn get_body(&self) -> &String {
    &self.body
  }

  pub fn new(status_code: &str, body: &str) -> RequestResponse {
    RequestResponse {
      status_code: String::from(status_code),
      body: String::from(body)
    } 
  }
}