extern crate chrono;
extern crate hyper;
extern crate websocket;

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

    fn request<'a, F: Fn() -> hyper::client::RequestBuilder<'a>>(&self, url: &str, f: F) -> Result<hyper::client::Response> {
        
    }
}
