#![feature(proc_macro_hygiene, decl_macro)]

use std::error::Error;
use std::any::Any;
use std::env;
use select::document::Document;
use rocket_contrib::json::Json;
use stremio_core::types::addons::*;
use stremio_core::types::*;
use select::predicate::Name;

#[macro_use] extern crate rocket;

const MANIFEST_RAW: &str = include_str!("../manifest.json");

#[get("/manifest.json")]
//#[response(content_type = "json")]
fn manifest() -> String {
    MANIFEST_RAW.into()
}

#[get("/catalog/channel/memes.json")]
fn catalog() -> Option<Json<ResourceResponse>> {
    Some(Json(
        get_videos()
            .map(|metas| ResourceResponse::Metas { metas })
            .ok()?,
    ))
}

fn get_videos() -> Result<Vec<MetaPreview>, Any> {
    let URL = String::from("https://www.googleapis.com/youtube/v3/search?key=");
    let KEY = String::from(env::var("API_KEY"));
    let QUERY = String::from("&q=memes+compilation&order=relevance&maxResults=50&relevanceLanguage=en&videoCategoryId=23&type=video&part=snippet&videoEmbeddable=true&safeSearch=strict&fields=prevPageToken,nextPageToken,items%2Fid%2FvideoId");

    let route = String::from(URL, KEY, QUERY);
    let resp = reqwest::get(route)?;
    if !resp.status().is_success() {
        return Err("request was not a success".into());
    };

    Ok(Document::from_read(resp)?
        .find(Name("pre"))
        .filter_map(|video| {
            let name = "Memes";
            Some(MetaPreview {
                id: video.id,
                poster: Some(String::from("Poster")),
                name,
                poster_shape: PosterShape::Portrait,
            })
        })
        .collect())

}

fn main() {
    rocket::ignite().mount("/", routes![manifest, catalog]).launch();
}