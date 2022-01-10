use super::utils;

#[test]
fn test_get_command_from_query_string_no_whitespace() {
    let actual = utils::get_command_from_query_string("tw");
    let expected = "tw";
    assert_eq!(actual, expected);
}

#[test]
fn test_get_command_from_query_string_with_whitespace() {
    let actual = utils::get_command_from_query_string("tw @fbOpenSource");
    let expected = "tw";
    assert_eq!(actual, expected);
}

// GitHub
#[test]
fn test_construct_github_profile_url_with_gh() {
    let fake_query = "gh";
    assert_eq!(
        utils::github::construct_github_url(fake_query),
        "https://github.com"
    );
}

#[test]
fn test_construct_github_profile_url_with_repo_url() {
    let fake_query = "gh facebook";
    assert_eq!(
        utils::github::construct_github_url(fake_query),
        "https://github.com/facebook"
    );
}

#[test]
fn test_construct_github_search_url_with_repo_url() {
    let fake_query = "gh facebook/docusaurus";
    assert_eq!(
        utils::github::construct_github_url(fake_query),
        "https://github.com/facebook/docusaurus"
    );
}

// Google Search
#[test]
fn test_construct_google_search_url() {
    let fake_query = "hello";
    assert_eq!(
        utils::google::construct_google_search_url(fake_query),
        "https://google.co.uk/search?q=hello"
    );
}

#[test]
fn test_construct_google_search_url_with_encoding() {
    let fake_query = "hello world";
    assert_eq!(
        utils::google::construct_google_search_url(fake_query),
        "https://google.co.uk/search?q=hello%20world"
    );
}

// Google Maps
#[test]
fn test_construct_googlemaps_url() {
    let fake_query = "gmaps";
    assert_eq!(
        utils::googlemaps::construct_googlemaps_url(fake_query),
        "https://www.google.co.uk/maps"
    );
}

#[test]
fn test_construct_googlemaps_search_url() {
    let fake_query = "gmaps london";
    assert_eq!(
        utils::googlemaps::construct_googlemaps_url(fake_query),
        "https://www.google.co.uk/maps/search/?api=1&query=london"
    );
}

#[test]
fn test_construct_googlemaps_coords_search_url_all() {
    let fake_query_roadmap = "gmaps 49.925619, 27.473947 z12 roadmap"; //https://www.google.com/maps/@49.925619,27.473947,12z
    let fake_query_terrain = "gmaps 49.925619, 27.473947 z12 terrain"; //https://www.google.com/maps/@49.925619,27.473947,12z/data=!5m1!1e4
    let fake_query_satellite = "gmaps 49.925619, 27.473947 z12 satellite"; //https://www.google.com/maps/@49.925619,27.473947,23299m/data=!3m1!1e3
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query_roadmap),
            "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=roadmap"
        );
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query_terrain),
            "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=terrain"
        );
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query_satellite),
            "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=satellite"
        )
}

#[test]
fn test_construct_googlemaps_coords_search_url_coords() {
    let fake_query = "gmaps 51.507395,-0.127709";
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query),
            "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=15&basemap=satellite"
        )
}

#[test]
fn test_construct_googlemaps_coords_search_url_coords_zoom() {
    let fake_query = "gmaps 51.507395,-0.127709 z18";
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query),
            "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=18&basemap=satellite"
        )
}

#[test]
fn test_construct_googlemaps_coords_search_url_coords_basemap() {
    let fake_query = "gmaps 51.507395,-0.127709 terrain";
    assert_eq!(
            utils::googlemaps::construct_googlemaps_url(fake_query),
            "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=15&basemap=terrain"
        )
}

// Myrotvorets
#[test]
fn test_construct_google_myrotvorets_search_url() {
    let fake_query = "myro https://ok.ru/profile/585163252505";
    assert_eq!(
        utils::myrotvorets::construct_google_myrotvorets_search_url(fake_query),
        "https://google.co.uk/search?q=https://ok.ru/profile/585163252505+site:myrotvorets.center"
    );
}

// Odnoklassniki
#[test]
fn test_construct_odnoklassniki_profile_url_with_ok() {
    let fake_query = "ok";
    assert_eq!(
        utils::odnoklassniki::construct_odnoklassniki_url(fake_query),
        "https://ok.ru"
    );
}

#[test]
fn test_construct_odnoklassniki_profile_url_with_id() {
    let fake_query = "ok id514836058554";
    assert_eq!(
        utils::odnoklassniki::construct_odnoklassniki_url(fake_query),
        "https://ok.ru/profile/514836058554"
    );
}

#[test]
fn test_construct_odnoklassniki_profile_url_with_username() {
    let fake_query = "ok iddmitry.dolyanovsky";
    assert_eq!(
        utils::odnoklassniki::construct_odnoklassniki_url(fake_query),
        "https://ok.ru/dmitry.dolyanovsky"
    );
}

#[test]
fn test_construct_odnoklassniki_search_url() {
    let fake_query = "ok АЛЕКСАНДР ЗАПОЛЬСКИЙ";
    assert_eq!(
            utils::odnoklassniki::construct_odnoklassniki_url(fake_query),
            "https://m.ok.ru/search/profiles/%D0%90%D0%9B%D0%95%D0%9A%D0%A1%D0%90%D0%9D%D0%94%D0%A0%20%D0%97%D0%90%D0%9F%D0%9E%D0%9B%D0%AC%D0%A1%D0%9A%D0%98%D0%99"
        );
}

// Twitter
#[test]
fn test_construct_twitter_url() {
    let fake_query = "tw";
    assert_eq!(
        utils::twitter::construct_twitter_url(fake_query),
        "https://twitter.com"
    );
}

#[test]
fn test_construct_twitter_url_query() {
    let fake_query = "tw hello world";
    assert_eq!(
        utils::twitter::construct_twitter_url(fake_query),
        "https://twitter.com/search?q=hello%20world"
    );
}

#[test]
fn test_construct_twitter_url_profile() {
    let fake_query = "tw @fbOpenSource";
    assert_eq!(
        utils::twitter::construct_twitter_url(fake_query),
        "https://twitter.com/fbOpenSource"
    );
}

#[test]
fn test_construct_twitter_profile_url() {
    let fake_profile = "jsjoeio";
    assert_eq!(
        utils::twitter::construct_twitter_profile_url(fake_profile),
        "https://twitter.com/jsjoeio"
    );
}

#[test]
fn test_construct_twitter_search_url() {
    let fake_query = "hello world";
    assert_eq!(
        utils::twitter::construct_twitter_search_url(fake_query),
        "https://twitter.com/search?q=hello%20world"
    );
}

// Vkontakte
#[test]
fn test_construct_vkontakte_profile_url_with_ok() {
    let fake_query = "vk";
    assert_eq!(
        utils::vkontakte::construct_vkontakte_url(fake_query),
        "https://vk.com"
    );
}

#[test]
fn test_construct_vkontakte_profile_url_with_id() {
    let fake_query = "vk id60890979";
    assert_eq!(
        utils::vkontakte::construct_vkontakte_url(fake_query),
        "https://vk.com/id60890979"
    );
}

#[test]
fn test_construct_vkontakte_profile_url_with_username() {
    let fake_query = "vk idkazak_lihoy";
    assert_eq!(
        utils::vkontakte::construct_vkontakte_url(fake_query),
        "https://vk.com/kazak_lihoy"
    );
}

#[test]
fn test_construct_vkontakte_search_url() {
    let fake_query = "vk Sergey Stratu";
    assert_eq!(
        utils::vkontakte::construct_vkontakte_url(fake_query),
        "https://vk.com/people/Sergey%20Stratu"
    );
}

// Yandex Maps
#[test]
fn test_construct_yandexmaps_url() {
    let fake_query = "ymaps";
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query),
        utils::yandexmaps::YANDEX_BASE_URL
    );
}

#[test]
fn test_construct_yandexmaps_search_url() {
    let fake_query = "ymaps london";
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query),
        "https://yandex.ru/maps/?text=london"
    );
}

#[test]
fn test_construct_yandexmaps_coords_search_url_all() {
    let fake_query_default = "ymaps 37.724664,55.750575 z18";
    let fake_query_map = "ymaps 37.724664,55.750575 z18 map";
    let fake_query_sat = "ymaps 37.724664,55.750575 z4 sat";
    let fake_query_skl = "ymaps 37.724664,55.750575 z20 hyb";
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query_default),
        "https://yandex.ru/maps/?ll=37.724664,55.750575&z=18&l=skl"
    );
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query_map),
        "https://yandex.ru/maps/?ll=37.724664,55.750575&z=18&l=map"
    );
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query_sat),
        "https://yandex.ru/maps/?ll=37.724664,55.750575&z=4&l=sat"
    );
    assert_eq!(
        utils::yandexmaps::construct_yandexmaps_url(fake_query_skl),
        "https://yandex.ru/maps/?ll=37.724664,55.750575&z=15&l=sat,skl"
    )
}

// Vkamico
// #[test]
// fn test_construct_vkamico_url_without_vkontakte_base_url() {
//     let fake_query = "vkid valera_666";
//     assert_eq!(
//         utils::vkamico::construct_vkamico_url(fake_query),
//         "https://vkamico.ru/valera_666"
//     )
// }

// #[test]
// fn test_construct_vkamico_url_with_vkontakte_base_url() {
//     let fake_query = "vkid https://vk.com/valera_666";
//     assert_eq!(
//         utils::vkamico::construct_vkamico_url(fake_query),
//         "https://vkamico.ru/valera_666"
//     )
// }

// Showid
#[test]
fn test_construct_showid_url_with_vkontakte_base_url() {
    let fake_query = "vkid https://vk.com/valera_666";
    assert_eq!(
        utils::vk_showid::construct_showid_url(fake_query),
        "http://showid.ru/names/valera_666"
    )
}

#[test]
fn test_construct_showid_url_without_vkontakte_base_url() {
    let fake_query = "vkid valera_666";
    assert_eq!(
        utils::vk_showid::construct_showid_url(fake_query),
        "http://showid.ru/names/valera_666"
    )
}
