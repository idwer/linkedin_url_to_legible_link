use std::env;

mod linkedin_url_to_legible_link;

use crate::linkedin_url_to_legible_link::decode_linkedin_url;
use crate::linkedin_url_to_legible_link::linkedin_url_is_valid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    if !linkedin_url_is_valid(url) {
        println!("Most likely you can already visit {url}");

        std::process::exit(0);
    }

    if let Some(s) = decode_linkedin_url(url) {
        println!("The actual URL is: {s}")
    }
}
