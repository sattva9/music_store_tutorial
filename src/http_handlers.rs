use actix_web::web;
use askama_actix::Template;

use crate::db::*;
use crate::music_error::MusicError;

#[derive(Template)]
#[template(path = "index.html")]
pub struct WelcomePage;

pub async fn welcome() -> WelcomePage {
    WelcomePage
}

#[derive(Template)]
#[template(path = "list_music.html")]
pub struct ListMusicPage {
    data: Vec<Music>,
}

pub async fn list_music() -> Result<ListMusicPage, MusicError> {
    Ok(ListMusicPage {
        data: music_db_data()?,
    })
}

#[derive(Template)]
#[template(path = "add_music.html")]
pub struct AddMusicPage;

pub async fn add_music() -> AddMusicPage {
    AddMusicPage
}

pub async fn add_music_to_db(data: web::Form<Music>) -> Result<web::Redirect, MusicError> {
    insert_data_to_db(data.into_inner())?;
    Ok(web::Redirect::to("/list_music").see_other())
}
