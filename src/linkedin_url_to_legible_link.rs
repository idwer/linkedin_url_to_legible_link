use urlencoding::decode;

pub fn decode_linkedin_url(url: &str) -> Option<String> {
    let result_cow_from_decode = decode(url);

    match result_cow_from_decode {
        Ok(o) => {
            let o_split = o.split("go?url=");
            let intermediate_split_elements = o_split.collect::<Vec<_>>();
            let intermediate_url = intermediate_split_elements[1];
            let split_elements = intermediate_url.split("&trk").collect::<Vec<_>>();

            return Some(split_elements[0].to_string())
        },
        _ => (),
    }

    None
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
