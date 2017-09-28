struct Config {
    client_id: String,
    client_secret: String,
    endpoint: String,
    redirect_uri: String,
    scopes: Vec<String>,
}

const AUTH_URL: &'static str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &'static str = "https://accounts.spotify.com/authorize";

#[allow(non_snake_case)]
mod Scopes {
    const SCOPE_IMAGE_UPLOAD: &'static str = "ugc-image-upload";
    const SCOPE_PLAYLIST_READ_PRIVATE: &'static str = "playlist-read-private";
    const SCOPE_PLAYLIST_MODIFY_PUBLIC
}

struct Authenticator {
    config: &Config,
}

impl Authenticator {
    pub fn new(&mut self, redirect: String, scopes: Vec<String>) -> Self {
        let cfg: Config;
        
        let client_id = match env::var("SPOTIFY_CLIENT") {
            Ok(i) => i,
            Err(err) => err,
        };

        let client_secret = match env::var("SPOTIFY_CLIENT_SECRET") {
            Ok(s) => s,
            Err(err) => err,
        };
        
        if client_id.is_err() && client_secret.is_err() {
            println!("Errors encountered while retrieving client identification. {} {}", client_id.unwrap_err().description(), client_secret.unwrap_err().description());
        } else {
            cfg = Config {
                client_id: client_id,
                client_secret: client_secret,
                endpoint: 
                redirect: redirect,
                scopes: scopes,
            }
        }

        Authenticator {
            config: &cfg,
        }
    }
}
