use std::env;

mod linkedin_url_to_legible_link;

use crate::linkedin_url_to_legible_link::decode_linkedin_url;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    
    if !url.starts_with("https://www.linkedin.com/safety/go?url=http") {
	    println!("Not a suitable link (does not start with 'https://www.linkedin.com/safety/go?url=http'), exiting");

	    std::process::exit(0);
    }    

    match decode_linkedin_url(url) {
        Some(s) => println!("The actual URL is: {}", s),
        _ => (),
    }
}
