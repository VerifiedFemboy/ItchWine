use std::{thread::sleep, time::Duration};

use itchwine_api::api::Api;

#[tokio::main]
async fn main() {
    let config = itchwine_api::oauth::OAuthConfig {
        client_id: "8bf9d5341a5f81827a493f84ac4e5a64".to_string(),
        redirect_uri: "http://localhost:8080/callback".to_string(),
    };

    config.open_browser();
    let api = Api::new().await;
    let me = api.get_me().await.unwrap();
    println!("Me: {:?}", me);
    sleep(Duration::from_secs(3));
    
}
