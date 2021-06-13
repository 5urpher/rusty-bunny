extern crate percent_encoding;

use lazy_static::lazy_static;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

// https://developers.google.com/maps/documentation/urls/get-started
// https://moz.com/blog/new-google-maps-url-parameters

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"[+-]?[0-9]{1,2}\.[0-9]{2,}\s?[NS]?°?[,\s]{1,2}[+-]?[0-9]{1,3}\.[0-9]{2,}\s?[EW]?°?",
    )
    .unwrap();
}

lazy_static! {
    static ref QUERY: Regex = Regex::new(
        r"([+-]?[0-9]{1,2}\.[0-9]{2,}\s?[NS]?°?[,\s]{1,2}[+-]?[0-9]{1,3}\.[0-9]{2,}\s?[EW]?°?)\s+(z[0-9]{2})\s+(\broadmap|\bterrain|\bsatellite)?",
    )
    .unwrap();
}

lazy_static! {
    static ref COORDS: Regex = Regex::new(
        r"[+-]?[0-9]{1,2}\.[0-9]{2,}\s?[NS]?°?[,\s]{1,2}[+-]?[0-9]{1,3}\.[0-9]{2,}\s?[EW]?°?",
    )
    .unwrap();
}

lazy_static! {
    static ref ZOOM_LEVEL: Regex = Regex::new(r"z[0-9]{2}",).unwrap();
}

lazy_static! {
    static ref BASEMAP: Regex = Regex::new(r"\broadmap|\bterrain|\bsatellite",).unwrap();
}

pub fn construct_googlemaps_url(query: &str) -> String {
    if query == "gmaps" {
        let googlemaps_dotcouk = "https://www.google.co.uk/maps";

        googlemaps_dotcouk.to_string()
    } else if COORDS.is_match(&query[6..]) {
        construct_googlemaps_coords_search_url(&query[6..])
    } else {
        construct_googlemaps_search_url(&query[6..])
    }
}

pub fn construct_googlemaps_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let googlemaps_search_url = format!(
        "https://www.google.co.uk/maps/search/?api=1&query={}",
        encoded_query
    );

    googlemaps_search_url
}

fn construct_googlemaps_coords_search_url(query: &str) -> String {
    let zoom_level;
    let basemap;

    let coords = COORDS.find(query).unwrap().as_str().trim();

    match ZOOM_LEVEL.find(query) {
        Some(mat) => zoom_level = &query[mat.start() + 1..mat.end()],
        None => zoom_level = "15",
    }

    match BASEMAP.find(query) {
        Some(bm) => basemap = bm.as_str(),
        None => basemap = "satellite",
    }

    let encoded_coords = utf8_percent_encode(coords, FRAGMENT).to_string();
    let googlemaps_coords_search_url = format!(
        "https://www.google.com/maps/@?api=1&map_action=map&center={}&zoom={}&basemap={}",
        encoded_coords, zoom_level, basemap
    );

    googlemaps_coords_search_url
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_construct_googlemaps_url() {
//         let fake_query = "gmaps";
//         assert_eq!(
//             construct_googlemaps_url(fake_query),
//             "https://www.google.co.uk/maps"
//         );
//     }

//     #[test]
//     fn test_construct_googlemaps_search_url() {
//         let fake_query = "gmaps london";
//         assert_eq!(
//             construct_googlemaps_url(fake_query),
//             "https://www.google.co.uk/maps/search/?api=1&query=london"
//         );
//     }

//     #[test]
//     fn test_construct_googlemaps_coords_search_url_all() {
//         let fake_query_roadmap = "gmaps 49.925619, 27.473947 z12 roadmap"; //https://www.google.com/maps/@49.925619,27.473947,12z
//         let fake_query_terrain = "gmaps 49.925619, 27.473947 z12 terrain"; //https://www.google.com/maps/@49.925619,27.473947,12z/data=!5m1!1e4
//         let fake_query_satellite = "gmaps 49.925619, 27.473947 z12 satellite"; //https://www.google.com/maps/@49.925619,27.473947,23299m/data=!3m1!1e3
//         assert_eq!(
//             construct_googlemaps_url(fake_query_roadmap),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=roadmap"
//         );
//         assert_eq!(
//             construct_googlemaps_url(fake_query_terrain),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=terrain"
//         );
//         assert_eq!(
//             construct_googlemaps_url(fake_query_satellite),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=49.925619,%2027.473947&zoom=12&basemap=satellite"
//         )
//     }

//     #[test]
//     fn test_construct_googlemaps_coords_search_url_coords() {
//         let fake_query = "gmaps 51.507395,-0.127709";
//         assert_eq!(
//             construct_googlemaps_url(fake_query),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=15&basemap=satellite"
//         )
//     }

//     #[test]
//     fn test_construct_googlemaps_coords_search_url_coords_zoom() {
//         let fake_query = "gmaps 51.507395,-0.127709 z18";
//         assert_eq!(
//             construct_googlemaps_url(fake_query),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=18&basemap=satellite"
//         )
//     }

//     #[test]
//     fn test_construct_googlemaps_coords_search_url_coords_basemap() {
//         let fake_query = "gmaps 51.507395,-0.127709 terrain";
//         assert_eq!(
//             construct_googlemaps_url(fake_query),
//             "https://www.google.com/maps/@?api=1&map_action=map&center=51.507395,-0.127709&zoom=15&basemap=terrain"
//         )
//     }
// }
