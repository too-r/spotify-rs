extern crate chrono;
extern crate hyper;
extern crate websocket;
extern crate reqwest;
extern crate base64;

use std::path::Path;
use std::io::prelude::*; //Import the Read trait and other stuff easily.
pub use chrono::prelude::*;

type Result<T> = Result<T, >;

//Our struct describing Spotify.
pub struct Spotify {
    token: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: i32,
    refresh_token: String,
}

impl Spotify {
    //Retrieve data we can use to authenticate with from the Spotify accounts service.
    fn get_token(path: Path) -> AuthResponse {
        #[derive(Deserialize)]
        struct AuthCode {
            code: String,
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

                let post_url = format!("grant_type=authorization_code code={} redirect_uri=localhost:8000", auth_struct.code);
                let encoded_secret = format!()

                let body = reqwest::Body::new(post_url).unwrap();

                let client = reqwest::Client::new().unwrap();
                let post = client.post()
                    .body(body)
                    .headers();
            }
        
            Err(err) => {
                println!("Receive error: {:?}", err);
                
                //Return a dummy response
                AuthResponse {
                    access_token: String::new(),
                    token_type: String::new(),
                    scope: String::new(),
                    expires_in: 1,
                    refresh_token: String::new(),
                }
            }
        }
    }
}
