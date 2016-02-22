crawl-destroyer
===

This is a spider trap to be used for finding an elusive segfault in grab-site or wpull.

1.	`git clone https://github.com/ludios/crawl-destroyer`

2.	Optionally compile from source if you don't want to use the pre-built binary in `./target/release/crawl-destroyer`:

	1.	[Install Rust Nightly](https://www.rust-lang.org/), which includes rustc and cargo.

	2.	`cd crawl-destroyer`

	3.	`cargo build --release`

3.	Start the web server with

	```
	while true; do ./target/release/crawl-destroyer; done
	```

	Need a while loop because it sometimes segfaults :-)

4.	Point crawlers to http://127.0.0.1:3000/

You can optionally pass an interface:port as the first argument to `crawl-destroyer`.
