use std::{borrow::Cow};

use urlencoding::decode;

pub fn decode_linkedin_url(url: &str) -> String {
    let d = decode(url).expect("");

    match d {
        Cow::Borrowed(e) => {
            return e.to_string()
        },

        Cow::Owned(ref e) => {
		let f = e.split_once('?');
		
		match f {
			None => (),
			Some(some) => {
				let s = some.1;
				let t: Vec<&str> = s.split("url=").collect();

				if t[1].contains("&trk=") {
					let u: Vec<&str> = t[1].split("&trk=").collect();
					return u[0].to_string();
				}			

				if t[1].contains("&vjs=") {
					let u: Vec<&str> = t[1].split("&vjs=").collect();
					return u[0].to_string();
				}
			}
		}
		
		return e.to_string()
        },
    }
}
