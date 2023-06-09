use once_cell::sync::Lazy;
use url::Url;

#[cfg(debug_assertions)]
pub const BENCHER_DEV: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
pub const BENCHER_DEV: &str = "https://bencher.dev";

#[allow(clippy::panic)]
pub static BENCHER_DEV_URL: Lazy<Url> = Lazy::new(|| {
    BENCHER_DEV
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse endpoint \"{BENCHER_DEV}\": {e}"))
});

pub fn is_bencher_dev(url: &Url) -> bool {
    *url == *BENCHER_DEV_URL
}
