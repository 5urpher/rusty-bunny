pub mod github;
pub mod google;
pub mod googlemaps;
pub mod myrotvorets;
pub mod odnoklassniki;
pub mod twitter;
pub mod vk_showid;
// pub mod vkamico;
pub mod vkontakte;
pub mod yandexmaps;

const VK_BASE_URL: &str = "https://vk.com/";
const VK_MOBILE_BASE_URL: &str = "https://m.vk.com/";

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap();
        return &query_string[..index_of_space];
    }
    query_string
}
