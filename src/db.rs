use crate::music_error::MusicError;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::*;

type DB = HashMap<u64, Music>;

lazy_static! {
    pub static ref MUSIC_DB: RwLock<DB> = Default::default();
}

#[derive(Default, Deserialize, Clone)]
pub struct Music {
    #[serde(skip_deserializing)]
    pub id: u64,
    pub song: String,
    pub artist: String,
    pub genre: String,
}

fn music_db_read() -> Result<RwLockReadGuard<'static, DB>, MusicError> {
    MUSIC_DB
        .read()
        .map_err(|e| MusicError::from(format!("{e}")))
}

fn music_db_write() -> Result<RwLockWriteGuard<'static, DB>, MusicError> {
    MUSIC_DB
        .write()
        .map_err(|e| MusicError::from(format!("{e}")))
}

pub fn music_db_data() -> Result<Vec<Music>, MusicError> {
    let data = music_db_read()?.clone().into_values().collect();
    Ok(data)
}

pub fn insert_data_to_db(mut music: Music) -> Result<(), MusicError> {
    let mut db = music_db_write()?;
    music.id = db.len() as u64;
    db.insert(music.id, music);
    Ok(())
}

pub fn get_data_by_id(id: u64) -> Result<Option<Music>, MusicError> {
    let db = music_db_read()?;
    let music = db.get(&id).map(|music| music.to_owned());
    Ok(music)
}

pub fn update_data_in_db(id: u64, mut music: Music) -> Result<(), MusicError> {
    let mut db = music_db_write()?;
    music.id = id;
    db.insert(id, music);
    Ok(())
}

pub fn delete_data_from_db(id: u64) -> Result<(), MusicError> {
    let mut db = music_db_write()?;
    db.remove(&id);
    Ok(())
}
