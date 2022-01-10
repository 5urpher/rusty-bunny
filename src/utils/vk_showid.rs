const SHOWID_BASE_URL: &str = "http://showid.ru/names/";

pub fn construct_showid_url(query: &str) -> String {
    if query.contains(super::VK_BASE_URL) {
        format!("{}{}", SHOWID_BASE_URL, &query[20..])
    } else {
        format!("{}{}", SHOWID_BASE_URL, &query[5..])
    }
}
