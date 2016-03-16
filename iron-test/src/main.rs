extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;


#[derive(RustcDecodable,RustcEncodable)]
struct Greeting {
  msg: String
}

fn main() {
  fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

  println!("On 3994");
  Iron::new(hello_world).http("0.0.0.0:3994").unwrap();
  
}
