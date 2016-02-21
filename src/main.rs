extern crate rand;
extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

use rand::{Rng, StdRng};

fn add_random_stuff(s: &mut String) -> &mut String {
	let mut rand = StdRng::new().unwrap();
	loop {
		match rand.gen_range(0, 100) {
			n if n == 0 => {
				break;
			}
			n if n == 1 => {
				s.push_str("<blockquote>");
				add_random_stuff(s);
				s.push_str("</blockquote>");
			}
			_ => {
				s.push_str("words<br>");
			}
		}
	}
	s
}

fn make_random_page() -> String {
	let mut s = String::new();
	s.push_str("<!DOCTYPE html>");
	add_random_stuff(&mut s);
	s
}

fn hello(_: Request, res: Response) {
	res.send(make_random_page().as_bytes()).unwrap();
}

fn main() {
	Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
