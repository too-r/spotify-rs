use std::sync::Mutex;
use std::collections::BTreeMap;

use hyper;

#[derive(Default)]
struct RateLimit {
    reset: i64,
    limit: i64,
    remaining: i64,
}

pub struct RateLimits {
    global: Mutex<RateLimit>,
    endpoints: Mutex<BTreeMap<String, RateLimit>>,
}

impl RateLimits {
    //Issue a premptive check to see whether we are being rate-limited.
    pub fn pre_check(&self, url: &str)
}

