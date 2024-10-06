use crate::{oauth, structs::me::Me};

pub struct Api {
    pub token: String,
}

impl Api {
    
    pub fn new() -> Self {
        Self {
            token: "".to_string()
        }
    }

    pub async fn get_me(&self) -> Result<Me, reqwest::Error> {
        let response = reqwest::get(format!("https://itch.io/api/1/{}/me", self.token)).await?;
        let response_text = response.text().await?;
        let json_value: serde_json::Value = serde_json::from_str(&response_text).unwrap();
        let struct_json = json_value.get("user").unwrap();

        let me = Me {
            id: struct_json.get("id").unwrap().as_i64().unwrap(),
            username: struct_json.get("username").unwrap().as_str().unwrap().to_string(),
            display_name: 
            if struct_json.get("display_name").unwrap().is_null() {
                None
            } else {
                Some(struct_json.get("display_name").unwrap().as_str().unwrap().to_string())
            },
            developer: struct_json.get("developer").unwrap().as_bool().unwrap(),
            press_user: struct_json.get("press_user").unwrap().as_bool().unwrap(),
            url: struct_json.get("url").unwrap().as_str().unwrap().to_string(),
            cover_url: 
            if struct_json.get("cover_url").unwrap().is_null() {
                None
            } else {
                Some(struct_json.get("cover_url").unwrap().as_str().unwrap().to_string())
            },
            gamer: struct_json.get("gamer").unwrap().as_bool().unwrap(),
        };


        Ok(me)
    }

    pub async fn get_token(&mut self) -> Self {
        Self {
            token: oauth::listen_for_token().await.unwrap()
        }
    }
}