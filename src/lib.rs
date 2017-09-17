extern crate chrono;
extern crate hyper;
extern crate websocket;
extern crate reqwest;

use std::path::Path;
use std::io::prelude::*; //Import the Read trait and other stuff easily.
pub use chrono::prelude::*;

macro_rule! api_concat {
    ($e:expr) => (concat!("https://api.spotify.com/v1", $e))
}

//Our struct describing Spotify.
pub struct Spotify {
    ratelimits: RateLimits,
    client: hyper::Client,
}

impl Spotify {
    //Create a new instance of the struct.
    pub fn new() -> Self {
        Spotify {
            ratelimits: RateLimits::new(),
            client: hyper::Client::new(),
        }
    }

    fn get_token(path: Path) -> String {
        #[derive(Deserialize)]
        struct AuthCode {
            code: String,
        }
        
        #[derive(Deserialize)]
        struct AuthResponse {
            access_token: String,
            token_type: String,
            scope: String,
            expires_in: i32,
            refresh_token: String,
        }

        let mut buf = String::new();
        
        //Open the file containing our client ID.
        let mut f = File::open(path).unwrap();

        let mut client_id = f.read_to_string(&mut buf).unwrap().trim(); //Get rid of the newline character. This assumes the file containing the client id only contains one line.
        
        //Format the request string using the client id.
        let mut request = format!("https://accounts.spotify.com/authorize/?client_id={}&response_type=code&redirect_uri=localhost:8000", client_id);

        //Request an access code.
        let mut resp = reqwest::get(request);

        match resp {
            Ok(r) => {
                let mut auth_struct = serde_json::from_str::<AuthCode>(&r).unwrap();
                
                let post_url = format!("grant_type=authorization_code code={} redirect_uri=localhost:8000")

                let body = reqwest::Body::new(post_url).unwrap();

                let client = reqwest::Client::new().unwrap();
                match client.post().body(body).send() {
                    Ok(auth) => {
                        let response = serde_json::from_str::<AuthReponse>(&auth).unwrap();
                        return (reponse.access_token, response.refresh_token).into_vec();
                    }
                    Err(e) => format!("{:?}", e)
                }
            }
        }
    }
}
