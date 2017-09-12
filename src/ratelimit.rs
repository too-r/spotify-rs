use std::sync::Mutex;
use std::collections::BTreeMap;

use hyper;

#[derive(Default)]
struct RateLimit {
    reset: i64,
    limit: i64,
    remaining: i64,
}

impl RateLimit {
    fn pre_check(&mut self) {
        //Return here if we are uninitalized.
        if self.limit == 0 { return }

        let difference = self.reset - Utc::now().timestamp();
        if difference < 0 {
            //If the reset is in the past, we assume that the reset did occur and we're ok. When
            //the response returns we'll know whether the rate-limit was reset for real.
            self.reset += 3; //We're good for 3 seconds.
            self.remaining = self.limit;
            return
        }

        //If we can't make requests, wait for a while.
        if self.remaining <= 0 {
            //Sleep for 900ms just in case the difference is off.
            let delay = difference as u64 * 1000 + 900;
            warn!("Pre-ratelimit: sleeping for {}ms", delay);
            ::sleep_ms(delay);
            return
        }

        //Deduct from the requests we have remaining.
        self.remaining -= 1;
    }

    fn post_update(&mut self, response: &hyper::client::Response) -> bool {
        match self.try_post_update(response) {
            Err(e) => {
                error!("rate limit checking error: {}", e);
                false
            }

            Ok(r) => r
        }
    
    }

    fn try_post_update()
}

pub struct RateLimits {
    global: Mutex<RateLimit>,
    endpoints: Mutex<BTreeMap<String, RateLimit>>,
}

impl RateLimits {
    //Issue a premptive check.
    pub fn pre_check(&self, url: &str) {
        self.global().lock().expect("Rate limits corrupted.").pre_check(); //Check global rate limits.
        if let Some(r1) = self.endpoints.lock().expect("Rate limits corrupted.").get_mut(url) {
            r1.pre_check(); //Check the endpoints.
        }
    }

    //Update our ratelimit count based on headers in the response given to use for the passed url.
    //Return true if we were ratelimited.
    pub fn post_update(&self, url: &str, response: &hyper::client::Response) -> bool {
        if response.headers.get_raw("X-Ratelimit-Global").is_some() {
            self.global.lock().expect("Rate limits corrupted.").post_update(response)
        } else {
            self.endpoints.lock().expect("Rate limits corrupted.")
                .entry(url.to_owned())
                .or_insert_with(RateLimit::default)
                .post_update(response)
        }
    }
}

