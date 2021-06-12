extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_myrotvorets_search_url(query: &str) -> String {
    let site: &str = "myrotvorets.center";
    let encoded_site = utf8_percent_encode(site, FRAGMENT).to_string();
    let encoded_query = utf8_percent_encode(&query[5..], FRAGMENT).to_string();
    let google_search_myrotvorets_url = format!(
        "https://google.co.uk/search?q={}+site:{}",
        encoded_query, encoded_site
    );

    google_search_myrotvorets_url
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_google_myrotvorets_search_url() {
        let fake_query = "myro https://ok.ru/profile/585163252505";
        assert_eq!(
            construct_google_myrotvorets_search_url(fake_query),
            "https://google.co.uk/search?q=https://ok.ru/profile/585163252505+site:myrotvorets.center"
        );
    }
}
