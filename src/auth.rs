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
