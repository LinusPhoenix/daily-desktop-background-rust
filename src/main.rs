#![windows_subsystem = "windows"]

use reqwest::blocking::Client;
use serde_json::Value;

const DESKTOP_BACKGROUND_ENDPOINT: &str =
    "https://api.unsplash.com/photos/random?query=desktop%20background&orientation=landscape";

fn main() {
    let access_key = load_access_key();
    let photo = get_desktop_background_from_unsplash(&access_key);

    wallpaper::set_from_url(&photo.image_url).unwrap();

    call_download_tracking_url(&photo);
}

fn load_access_key() -> String {
    let access_key = include_bytes!(".access-key");
    String::from_utf8_lossy(access_key).into_owned()
}

#[derive(Debug)]
struct UnsplashPhoto {
    image_url: String,
    download_tracking_url: String,
}

fn get_desktop_background_from_unsplash(access_key: &str) -> UnsplashPhoto {
    let http_client = Client::new();
    let response = http_client
        .get(DESKTOP_BACKGROUND_ENDPOINT)
        .header("Authorization", format!("Client-ID {}", access_key))
        .send()
        .unwrap();
    let response: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let image_url = response["urls"]["full"].as_str().unwrap();
    let download_tracking_url = response["links"]["download_location"].as_str().unwrap();

    UnsplashPhoto {
        image_url: String::from(image_url),
        download_tracking_url: String::from(download_tracking_url),
    }
}

fn call_download_tracking_url(photo: &UnsplashPhoto) {
    reqwest::blocking::get(&photo.download_tracking_url).unwrap();
}
