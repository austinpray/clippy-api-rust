extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{Nickel, Request, Response, HttpRouter};
use std::io::net::ip::Ipv4Addr;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    fn hello_handler(request: &Request, response: &mut Response) {
        let hello_thing = format!("Hello {}!", request.param("thing"));
        response.send(hello_thing.as_slice());
    }

    router.get("/hello/:thing", hello_handler);

    server.utilize(router);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
