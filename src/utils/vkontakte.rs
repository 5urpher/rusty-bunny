extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_vkontakte_url(query: &str) -> String {
    if query == "vk" {
        let vkontakte_dotru = "https://vk.com";

        vkontakte_dotru.to_string()
    } else if &query[..5] == "vk id" {
        construct_vkontakte_profile_url(&query[5..])
    } else {
        construct_vkontakte_search_url(&query[3..])
    }
}

pub fn construct_vkontakte_profile_url(profile: &str) -> String {
    if profile.parse::<i64>().is_ok() {
        format!("https://vk.com/id{}", &profile)
    } else {
        format!("https://vk.com/{}", profile)
    }
}

pub fn construct_vkontakte_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let vkontakte_search_url = format!("https://vk.com/people/{}", encoded_query);

    vkontakte_search_url
}
