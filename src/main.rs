extern crate url;
extern crate html5ever;

use std::env;
use std::io::stdout;
use std::io::Write;
use url::Url;

use fetching::UrlState;
use std::process;


mod fetching;
mod parsing;
mod crawling;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 3 {
        let start_url_string = &args[1];
        let n_threads = args[2].parse::<i32>().unwrap();
        let n_max_to_crawl = args[3].parse::<i32>().unwrap();

        // TODO: a proper error message here.
        let start_url = Url::parse(start_url_string).unwrap();

        let domain = start_url.domain().expect("I can't find a domain in your URL");
        let path_components = start_url.path().expect("I can't find a path in your URL");

        let mut success_count = 0;
        let mut fail_count = 0;

        for url_state in crawling::crawl(&domain, &path_components.join("/"), n_threads) {
            match url_state {
                UrlState::Accessible(_) => {
                    success_count += 1;
                }
                status => {
                    fail_count += 1;
                    // println!("{}", status);
                }
            }

            print!("Succeeded: {} Failed: {}\r",
                   success_count,
                   fail_count);
            stdout().flush().unwrap();

            if success_count >= n_max_to_crawl {
                process::exit(0);
            }
        }

    } else {
        // TODO: exit non-zero and print proper usage.
        println!("Please provide an URL.")
    }
}
