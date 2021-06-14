pub fn construct_vkamico_url(query: &str) -> String {
    if query.contains("https://vk.com/") {
        format!("https://vkamico.ru/{}", &query[20..])
    } else {
        format!("https://vkamico.ru/{}", &query[5..])
    }
}
