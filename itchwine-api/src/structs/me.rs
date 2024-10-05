use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub id: i32,
    pub username: String,
    pub developer: bool,
    pub press_user: bool,
    pub url: String,
    pub gamer: bool,
}