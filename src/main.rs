extern crate rand;
extern crate hyper;

use std::env;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::header::{Connection, ContentType};
use rand::{Rng, StdRng};

fn add_random_stuff<'a>(depth: u32, elems: &mut u32, s: &'a mut String) -> &'a mut String {
	if depth > 30 {
		return s;
	}
	let mut rand = StdRng::new().unwrap();
	loop {
		if *elems > 5000 {
			break;
		}
		match rand.gen_range(0, 100) {
			n if n == 0 => {
				break;
			}
			n if n < 10 => {
				*elems += 1;
				s.push_str("<blockquote>");
				add_random_stuff(depth + 1, elems, s);
				s.push_str("</blockquote>");
			}
			n if n < 12 => {
				*elems += 1;
				let n_bytes = rand.gen_range(0, 12000);
				let text: String = rand.gen_ascii_chars().take(n_bytes).collect();
				s.push_str(&format!(r#"<a href="{}">{}</a><br>"#, text, text));
			}
			n if n < 50 => {
				*elems += 1;
				let n_bytes = rand.gen_range(0, 9);
				let text: String = rand.gen_ascii_chars().take(n_bytes).collect();
				s.push_str(&format!(r#"<a href="{}">{}</a><br>"#, text, text));
			}
			n if n < 100 => {
				*elems += 1;
				let n_bytes = rand.gen_range(0, 9);
				let random_text: String = rand.gen_ascii_chars().take(n_bytes).collect();
				s.push_str("lots of words ");
				s.push_str(&random_text);
				match rand.gen_range(0, 2) {
					0 => s.push_str("<br>"),
					_ => s.push_str("<p>")
				}
			}
			_ => {
				*elems += 1;
				s.push_str("words<br>");
			}
		}
	}
	s
}

fn make_random_page() -> String {
	let mut s = String::new();
	s.push_str("<!DOCTYPE html>");
	s.push_str("<html>");
	s.push_str("<head>");
	s.push_str("</head>");
	s.push_str("<body>");
	add_random_stuff(0, &mut 0, &mut s);
	s.push_str("</body>");
	s.push_str("</html>");
	s
}

fn hello(_: Request, mut res: Response) {
	res.headers_mut().set(ContentType::html());
	// Need to close connections to serve > 5 simultaneous users (or at least wpull clients)
	// https://github.com/hyperium/hyper/issues/368
	res.headers_mut().set(Connection::close());
	res.send(make_random_page().as_bytes()).unwrap();
}

fn main() {
	let interface_port: String = env::args().nth(1).unwrap_or("127.0.0.1:3000".to_owned());
	println!("Listening on {}", interface_port);
	Server::http(&interface_port[..]).unwrap().handle(hello);
}
