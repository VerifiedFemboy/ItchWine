use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub developer: bool,
    pub press_user: bool,
    pub url: String,
    pub cover_url: Option<String>,
    pub gamer: bool,
}