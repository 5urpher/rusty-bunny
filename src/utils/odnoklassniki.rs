extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_odnoklassniki_url(query: &str) -> String {
    if query == "ok" {
        let odnoklassniki_dotru = "https://ok.ru";

        odnoklassniki_dotru.to_string()
    } else if &query[..5] == "ok id" {
        construct_odnoklassniki_profile_url(&query[5..])
    } else {
        construct_odnoklassniki_search_url(&query[3..])
    }
}

pub fn construct_odnoklassniki_profile_url(profile: &str) -> String {
    if profile.parse::<i64>().is_ok() {
        format!("https://ok.ru/profile/{}", &profile)
    } else {
        format!("https://ok.ru/{}", profile)
    }
}

pub fn construct_odnoklassniki_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let odnoklassniki_search_url = format!("https://m.ok.ru/search/profiles/{}", encoded_query);

    odnoklassniki_search_url
}
