use crate::{oauth, structs::me::Me};

pub struct Api {
    pub token: String,
}

impl Api {
    
    pub async fn new() -> Self {
        let token = oauth::listen_for_token().await.unwrap();
        Self {
            token,
        }
    }

    pub async fn get_me(&self) -> Result<Me, reqwest::Error> {
        let response = reqwest::get(format!("https://itch.io/api/1/{}/me", self.token)).await?;
        let response_text = response.text().await?;
        let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();
        let me: Me = serde_json::from_value(json_value["user"].clone()).unwrap();
        Ok(me)
    }

}