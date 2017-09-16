extern crate chrono;
extern crate hyper;
extern crate websocketi;
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
        let mut buf = String::new();
        
        //Open the file containing our client ID.
        let mut f = File::open(path).unwrap();

        let mut client_id = f.read_to_string(&mut buf).unwrap().trim(); //Get rid of the newline character. This assumes the file containing the client id has only contains one line.
        
        //Format the request string using the client id.
        let mut request = format!("https://accounts.spotify.com/authorize/?client_id={}&response_type=code&redirect_uri=https%3A%2F%2Flocalhost:8000", client_id);

        //Request an access code.
        let mut resp = reqwest::get(request);

        match resp {
            Ok(r) => {
                let mut 
            }
        }
    }
}
