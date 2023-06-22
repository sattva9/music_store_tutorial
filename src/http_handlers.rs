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

#[derive(Template)]
#[template(path = "edit_music.html")]
pub struct EditMusicPage {
    music: Music,
}

pub async fn edit_music(id: web::Path<u64>) -> Result<EditMusicPage, MusicError> {
    let music = get_data_by_id(id.into_inner())?
        .ok_or_else(|| MusicError::from("Music for id {id} doesn't exist in DB".to_string()))?;
    Ok(EditMusicPage { music })
}

pub async fn update_music(
    id: web::Path<u64>,
    data: web::Form<Music>,
) -> Result<web::Redirect, MusicError> {
    update_data_in_db(id.into_inner(), data.into_inner())?;
    Ok(web::Redirect::to("/list_music").see_other())
}

pub async fn delete_music(id: web::Path<u64>) -> Result<web::Redirect, MusicError> {
    delete_data_from_db(id.into_inner())?;
    Ok(web::Redirect::to("/list_music").see_other())
}
