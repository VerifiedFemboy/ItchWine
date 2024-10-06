use std::{thread::sleep, time::Duration};

use itchwine_api::api::Api;
use std::fs::File;
use std::io::Write;
use std::io::Read;

#[tokio::main]
async fn main() {
    let config = itchwine_api::oauth::OAuthConfig {
        client_id: "8bf9d5341a5f81827a493f84ac4e5a64".to_string(),
        redirect_uri: "http://localhost:8080/callback".to_string(),
    };

    let token_path = "/run/media/verifiedfemboy/Disk/programming/Rust/ItchWine/itchwine-tui/token.txt";

    let mut api = Api::new();
    if let Ok(mut file) = File::open(token_path) {
        let mut token = String::new();
        file.read_to_string(&mut token).unwrap();
        println!("Token found: {}", token);
        api.token = token;
    } else {
        config.open_browser();
        api = api.get_token().await;
        let mut file = File::create(token_path).unwrap();
        let placeholder_token = api.token.clone();
        file.write_all(placeholder_token.as_bytes()).unwrap();
        println!("Token file created with placeholder token.");
    }


    let me = api.get_me().await.unwrap();
    println!("Me: {:?}", me);
    sleep(Duration::from_secs(3));
    
}
