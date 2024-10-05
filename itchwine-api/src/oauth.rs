use std::collections::HashMap;

use warp::Filter;
use std::sync::{Arc, Mutex};

pub struct OAuthConfig {
    pub client_id: String,
    pub redirect_uri: String,
}

impl OAuthConfig {
    pub fn get_auth_url(&self) -> String {
        let scopes = "profile:me";
        format!("https://itch.io/user/oauth?client_id={}&scope={scopes}&response_type=token&redirect_uri={}",
            self.client_id, self.redirect_uri)
    }

    pub fn open_browser(&self) {
        if let Err(e) = webbrowser::open(&self.get_auth_url()) {
            eprintln!("Failed to open browser: {}", e);
        }
    }
}

pub async fn listen_for_token() -> Option<String> {
    let token = Arc::new(Mutex::new(None));
    let token_clone = Arc::clone(&token);

    let route = warp::path("callback")
        .and(warp::query::<HashMap<String, String>>())
        .map(move |params: HashMap<String, String>| {
            if let Some(fragment) = params.get("access_token") {
                let mut token = token_clone.lock().unwrap();
                *token = Some(fragment.clone());
                println!("Access token received: {}", fragment);
                return warp::reply::html(format!("Access token received: {}", fragment));
            }
            warp::reply::html(r#"
                <html>
                    <body>
                        <script type="text/javascript">
                            const fragment = new URLSearchParams(window.location.hash.substring(1));
                            const token = fragment.get('access_token');
                            if (token) {
                                window.location.href = window.location.pathname + '?' + new URLSearchParams({ access_token: token });
                            }
                        </script>
                        <p>Processing...</p>
                    </body>
                </html>
            "#.to_string())
        });

    let (_addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 8080));
    tokio::task::spawn(server);

    loop {
        if let Some(token) = token.lock().unwrap().clone() {
            return Some(token);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
