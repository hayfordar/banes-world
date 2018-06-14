extern crate gotham;
extern crate hyper;

extern crate mime;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod responses;
mod router;
mod routes;

fn main() {
    let addr = "127.0.0.1:6969";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router::router())
}