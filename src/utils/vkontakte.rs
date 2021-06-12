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
// https://m.vk.com/search?c[section]=people&c[name]=1&c[q]=
// https://vk.com/people/
pub fn construct_vkontakte_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let vkontakte_search_url = format!(
        // "https://m.vk.com/search?c[section]=people&c[name]=1&c[iphone]=1&c[country]=0&q={}",
        // "https://m.vk.com/search?c[section]=people&c[name]=1&c[q]={}",
        "https://vk.com/people/{}",
        encoded_query
    );

    vkontakte_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_vkontakte_profile_url_with_ok() {
        let fake_query = "vk";
        assert_eq!(construct_vkontakte_url(fake_query), "https://vk.com");
    }

    #[test]
    fn test_construct_vkontakte_profile_url_with_id() {
        let fake_query = "vk id60890979";
        assert_eq!(
            construct_vkontakte_url(fake_query),
            "https://vk.com/id60890979"
        );
    }

    #[test]
    fn test_construct_vkontakte_profile_url_with_username() {
        let fake_query = "vk idkazak_lihoy";
        assert_eq!(
            construct_vkontakte_url(fake_query),
            "https://vk.com/kazak_lihoy"
        );
    }

    #[test]
    fn test_construct_vkontakte_search_url() {
        let fake_query = "vk Sergey Stratu";
        assert_eq!(
            construct_vkontakte_url(fake_query),
            "https://m.vk.com/search?c[section]=people&c[name]=1&c[iphone]=1&c[country]=0&q=Sergey%20Stratu"
        );
    }
}
