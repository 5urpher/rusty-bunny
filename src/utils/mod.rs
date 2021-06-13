pub mod github;
pub mod google;
pub mod googlemaps;
pub mod myrotvorets;
pub mod odnoklassniki;
pub mod twitter;
pub mod vkontakte;
pub mod yandexmaps;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap();
        return &query_string[..index_of_space];
    }
    query_string
}
