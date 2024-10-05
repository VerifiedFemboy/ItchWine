pub mod oauth;
pub mod api;
pub mod structs;

pub async fn get_token() -> Option<String> {
    let token = oauth::listen_for_token().await;
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn oauth() {
        let config = oauth::OAuthConfig {
            client_id: "8bf9d5341a5f81827a493f84ac4e5a64".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
        };

        println!("Auth URL: {}", config.get_auth_url());
        println!("Listening for token...");
        let token = oauth::listen_for_token().await.unwrap();
        println!("Token received: {:?}", token);
    }
}
