use rocket::fs::NamedFile;
use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

mod utils;

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()
}

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
        // "gh" => utils::github::construct_github_url(&cmd),
        // "tw" => utils::twitter::construct_twitter_url(&cmd),
        "ok" => utils::odnoklassniki::construct_odnoklassniki_url(&cmd),
        "vk" => utils::vkontakte::construct_vkontakte_url(&cmd),
        "myro" => utils::myrotvorets::construct_google_myrotvorets_search_url(&cmd),
        "gmaps" => utils::googlemaps::construct_googlemaps_url(&cmd),
        "ymaps" => utils::yandexmaps::construct_yandexmaps_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     rocket::build()
//         .mount("/", routes![index, search, favicon])
//         .ignite()
//         .await?
//         .launch()
//         .await
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search, favicon])
}
