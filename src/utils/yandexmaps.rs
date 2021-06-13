// https://yandex.com/dev/yandex-apps-launch/maps/doc/concepts/yandexmaps-web.html/

use lazy_static::lazy_static;
use regex::Regex;

pub(crate) const YANDEX_BASE_URL: &str = "https://yandex.ru/maps/";
const DEFAULT_ZOOM_LEVEL: &str = "15";
const DEFAULT_BASEMAP: &str = "skl";

lazy_static! {
    static ref COORDS: Regex = Regex::new(
        r"[+-]?[0-9]{1,2}\.[0-9]{2,}\s?[NS]?°?[,\s]{1,2}[+-]?[0-9]{1,3}\.[0-9]{2,}\s?[EW]?°?",
    )
    .unwrap();
}

lazy_static! {
    static ref ZOOM_LEVEL: Regex = Regex::new(r"z[0-9]{1,2}",).unwrap();
}

lazy_static! {
    static ref BASEMAP: Regex = Regex::new(r"\bmap|\bsat|\bhyb",).unwrap();
}

pub fn construct_yandexmaps_url(query: &str) -> String {
    if query == "ymaps" {
        let yandexmaps_dotru = YANDEX_BASE_URL;

        yandexmaps_dotru.to_string()
    } else if COORDS.is_match(&query[6..]) {
        construct_yandexmaps_coords_search_url(&query[6..])
    } else {
        construct_yandexmaps_search_url(&query[6..])
    }
}

pub fn construct_yandexmaps_search_url(query: &str) -> String {
    let yandexmaps_search_url = format!("{}?text={}", YANDEX_BASE_URL, query);

    yandexmaps_search_url
}

fn construct_yandexmaps_coords_search_url(query: &str) -> String {
    let coords = COORDS.find(query).unwrap().as_str().trim();

    let zoom_level = match ZOOM_LEVEL.is_match(query) {
        true => {
            let mat = ZOOM_LEVEL.find(query).unwrap();
            let zoom = &query[mat.start() + 1..mat.end()];
            let zoom_parsed = zoom.parse::<u8>().expect("Expected u8");

            match zoom_parsed {
                1..=19 => zoom,
                _ => DEFAULT_ZOOM_LEVEL,
            }
        }
        _ => DEFAULT_ZOOM_LEVEL,
    };

    let basemap = match BASEMAP.is_match(query) {
        true => {
            let mat = BASEMAP.find(query).unwrap();
            let map = &query[mat.start()..mat.end()];

            match map {
                "hyb" => "sat,skl",
                _ => map,
            }
        }
        _ => DEFAULT_BASEMAP,
    };

    let yandexmaps_coords_search_url = format!(
        "{}?ll={}&z={}&l={}",
        YANDEX_BASE_URL, coords, zoom_level, basemap
    );

    yandexmaps_coords_search_url
}
