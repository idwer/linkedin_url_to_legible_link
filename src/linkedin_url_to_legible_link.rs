use urlencoding::decode;

pub fn linkedin_url_is_invalid(url: &str) -> bool {
    !url.starts_with("https://www.linkedin.com/safety/go?url=http")
}

fn split_url_crude(url: &str) -> Vec<&str> {
    url.split('&').collect::<Vec<_>>()
}

fn split_url_fine(url: &str) -> Vec<&str> {
    url.split("go?url=").collect::<Vec<_>>()
}

pub fn decode_linkedin_url(url: &str) -> Option<String> {
    let result_cow_from_decode = decode(url);

    if let Ok(decoded_url) = result_cow_from_decode {
        let vec_crude = split_url_crude(&decoded_url);
        let url_crude = vec_crude[0];

        let vec_fine = split_url_fine(url_crude);
        let url_fine = vec_fine[1];

        return Some(url_fine.to_string());
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

        assert!(!result);
    }

    #[test]
    fn test_split_url_crude() {
        let url_to_decode = "https://www.linkedin.com/safety/go?url=https://nos.nl/&trk=flagship-messaging-web&messageThreadUrn=urn:li:messagingThread:_redacted_hash==&lipi=urn:li:page:d_flagship3_messaging_conversation_detail;_faux_content==";
        let actual_url = split_url_crude(url_to_decode);

        assert_eq!(
            actual_url,
            [
                "https://www.linkedin.com/safety/go?url=https://nos.nl/",
                "trk=flagship-messaging-web",
                "messageThreadUrn=urn:li:messagingThread:_redacted_hash==",
                "lipi=urn:li:page:d_flagship3_messaging_conversation_detail;_faux_content=="
            ]
        );
    }

    #[test]
    fn test_split_url_fine() {
        let url_to_decode = "https://www.linkedin.com/safety/go?url=https://nos.nl/";
        let actual_url = split_url_fine(url_to_decode);

        assert_eq!(
            actual_url,
            ["https://www.linkedin.com/safety/", "https://nos.nl/"]
        );
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
