extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::utils::VK_BASE_URL;
use crate::utils::VK_MOBILE_BASE_URL;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_vkontakte_url(query: &str) -> String {
    if query == "vk" {
        VK_BASE_URL.to_string()
    } else if &query[..5] == "vk id" {
        construct_vkontakte_profile_url(&query[5..])
    } else {
        construct_vkontakte_search_url(&query[3..])
    }
}

pub fn construct_vkontakte_profile_url(profile: &str) -> String {
    if profile.parse::<i64>().is_ok() {
        format!("{}id{}", VK_BASE_URL, &profile)
    } else {
        format!("{}{}", VK_BASE_URL, profile)
    }
}

pub fn construct_vkontakte_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    // let vkontakte_search_url = format!("{}people/{}", VK_BASE_URL, encoded_query);
    let vkontakte_search_url = format!(
        "{}search?c[section]=people&c[name]=1&c[q]={}",
        VK_MOBILE_BASE_URL, encoded_query
    );

    vkontakte_search_url
}
// https://m.vk.com/search?c[section]=people&c[name]=1&c[q]=%D0%9E%D1%80%D0%B5%D1%89%D0%B5%D0%BD%D0%BA%D0%BE%D0%B2%20%D0%92%D0%B8%D0%BA%D1%82%D0%BE%D1%80
// https://m.vk.com/search?c[section]=people&c[name]=1&c[q]=Орещенков Виктор
