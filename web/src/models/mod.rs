use serde::{Deserialize, Serialize};
use validator::Validate;

pub const INSERT_BOOKMARK: &str = "INSERT INTO bookmarks (url, title) VALUES (?, ?);";

pub const SELECT_ALL_BOOKMARKS: &str = "SELECT id, url, title FROM bookmarks ORDER BY created_at DESC;";


#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
}