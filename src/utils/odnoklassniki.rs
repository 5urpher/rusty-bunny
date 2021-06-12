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

// https://m.ok.ru/search/profiles/
pub fn construct_odnoklassniki_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let odnoklassniki_search_url = format!("https://m.ok.ru/search/profiles/{}", encoded_query);

    odnoklassniki_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_odnoklassniki_profile_url_with_ok() {
        let fake_query = "ok";
        assert_eq!(construct_odnoklassniki_url(fake_query), "https://ok.ru");
    }

    #[test]
    fn test_construct_odnoklassniki_profile_url_with_id() {
        let fake_query = "ok id514836058554";
        assert_eq!(
            construct_odnoklassniki_url(fake_query),
            "https://ok.ru/profile/514836058554"
        );
    }

    #[test]
    fn test_construct_odnoklassniki_profile_url_with_username() {
        let fake_query = "ok iddmitry.dolyanovsky";
        assert_eq!(
            construct_odnoklassniki_url(fake_query),
            "https://ok.ru/dmitry.dolyanovsky"
        );
    }

    #[test]
    fn test_construct_odnoklassniki_search_url() {
        let fake_query = "ok АЛЕКСАНДР ЗАПОЛЬСКИЙ";
        assert_eq!(
            construct_odnoklassniki_url(fake_query),
            "https://m.ok.ru/search/profiles/%D0%90%D0%9B%D0%95%D0%9A%D0%A1%D0%90%D0%9D%D0%94%D0%A0%20%D0%97%D0%90%D0%9F%D0%9E%D0%9B%D0%AC%D0%A1%D0%9A%D0%98%D0%99"
        );
    }
}
