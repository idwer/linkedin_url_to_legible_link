use std::{borrow::Cow};

use urlencoding::decode;

pub fn decode_linkedin_url(url: &str) -> String {
    let d = decode(url).expect("");

    match d {
        Cow::Borrowed(e) => {
            e.to_string()
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
		
		e.to_string()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_linkedin_url() {
        let url_to_decode = "https://www.linkedin.com/safety/go?url=https%3A%2F%2Fnos.nl%2F&trk=flagship-messaging-web&messageThreadUrn=urn%3Ali%3AmessagingThread%3A_redacted_hash%3D%3D&lipi=urn%3Ali%3Apage%3Ad_flagship3_messaging_conversation_detail%3B_faux_content%3D%3D";
        let url = decode_linkedin_url(url_to_decode);

        assert_eq!(url, "https://nos.nl/");
    }
}
