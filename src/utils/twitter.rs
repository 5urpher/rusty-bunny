extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_twitter_url(query: &str) -> String {
    if query == "tw" {
        let twitter_dotcom = "https://twitter.com";

        twitter_dotcom.to_string()
    } else if &query[..4] == "tw @" {
        construct_twitter_profile_url(&query[4..])
    } else {
        construct_twitter_search_url(&query[3..])
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

pub fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let twitter_search_url = format!("https://twitter.com/search?q={}", encoded_query);

    twitter_search_url
}
