#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod utils;

use rocket::fs::NamedFile;
use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "
    RUSTY BUNNY
    
    USAGE

        TODO
    "
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "ok" => utils::odnoklassniki::construct_odnoklassniki_url(&cmd),
        "vk" => utils::vkontakte::construct_vkontakte_url(&cmd),
        "myro" => utils::myrotvorets::construct_google_myrotvorets_search_url(&cmd),
        "gmaps" => utils::googlemaps::construct_googlemaps_url(&cmd),
        "ymaps" => utils::yandexmaps::construct_yandexmaps_url(&cmd),
        // "vkid" => utils::vkamico::construct_vkamico_url(&cmd),
        "vkid" => utils::vk_showid::construct_showid_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search, favicon])
}
