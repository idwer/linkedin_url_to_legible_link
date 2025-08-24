use urlencoding::decode;

pub fn linkedin_url_is_invalid(url: &str) -> bool {
    return !url.starts_with("https://www.linkedin.com/safety/go?url=http")
}

pub fn decode_linkedin_url(url: &str) -> Option<String> {
    let result_cow_from_decode = decode(url);

    if let Ok(o) = result_cow_from_decode {
        let o_split = o.split("go?url=");
        let intermediate_split_elements = o_split.collect::<Vec<_>>();
        let intermediate_url = intermediate_split_elements[1];
        let split_elements = intermediate_url.split("&trk").collect::<Vec<_>>();

        return Some(split_elements[0].to_string())
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkedin_url_is_valid() {
        let url = "https://www.linkedin.com/safety/go?url=https://nos.nl/&trk=flagship-messaging-web&messageThreadUrn=urn:li:messagingThread:_redacted_hash==&lipi=urn:li:page:d_flagship3_messaging_conversation_detail;_faux_content==";
        let result = linkedin_url_is_invalid(url);

        assert_eq!(result, false);
    }

    #[test]
    fn test_decode_linkedin_url() {
        let url_to_decode = "https://www.linkedin.com/safety/go?url=https%3A%2F%2Fnos.nl%2F&trk=flagship-messaging-web&messageThreadUrn=urn%3Ali%3AmessagingThread%3A_redacted_hash%3D%3D&lipi=urn%3Ali%3Apage%3Ad_flagship3_messaging_conversation_detail%3B_faux_content%3D%3D";
        let option_intermediate_url = decode_linkedin_url(url_to_decode);

        let mut actual_url = String::new();

        if let Some(s) = option_intermediate_url {
            actual_url = s;
        }

        assert_eq!(actual_url, "https://nos.nl/");
    }
}
