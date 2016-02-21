extern crate rand;
extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

use rand::{Rng, StdRng};

fn add_random_stuff<'a>(depth: u32, elems: &mut u32, s: &'a mut String) -> &'a mut String {
	if depth > 30 {
		return s;
	}
	let mut rand = StdRng::new().unwrap();
	loop {
		if *elems > 500000 {
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
			n if n < 50 => {
				*elems += 1;
				let text: String = rand.gen_ascii_chars().take(10).collect();
				s.push_str(&format!(r#"<a href="{}">{}</a><br>"#, text, text));
			}
			n if n < 100 => {
				*elems += 1;
				s.push_str("lots of words<br>");
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

fn hello(_: Request, res: Response) {
	res.send(make_random_page().as_bytes()).unwrap();
}

fn main() {
	Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
