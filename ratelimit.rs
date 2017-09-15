#[derive(Default)]
pub struct RateLimits {
    global: Mutex<RateLimit>,
    endpoints: Mutex<BTreeMap<String, RateLimit>>,
}

#[derive(Default)]
struct RateLimit {
    reset: i64,
    limit: i64,
    remaining: i64,
}

impl RateLimit {
    fn pre_check(&mut self) {
        if self.limit == 0 { return }

        let difference = self.reset - Utc::now()
    }
}
