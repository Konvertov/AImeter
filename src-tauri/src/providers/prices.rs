//! Public API list prices, dollars per million tokens. Last checked 2026-11-03.
//! Used only to estimate what the tokens in local logs would cost at API
//! rates; a subscription is billed differently. A model that is not listed
//! here is left out of the estimate rather than guessed.

pub struct Price {
    pub input: f64,
    pub output: f64,
    pub cache_read: f64,
}

/// Name beginnings, most specific first ("opus-5-5" must come before "opus-5").
const TABLE: &[(&str, Price)] = &[
    ("fable-5-2", Price { input: 8.0, output: 40.0, cache_read: 0.20 }),
    ("mythos-5-2", Price { input: 8.0, output: 40.0, cache_read: 0.20 }),
    ("fable-5-1", Price { input: 10.0, output: 50.0, cache_read: 0.25 }),
    ("mythos-5-1", Price { input: 10.0, output: 50.0, cache_read: 0.25 }),
    ("fable-5", Price { input: 10.0, output: 50.0, cache_read: 1.0 }),
    ("opus-5-5", Price { input: 4.0, output: 20.0, cache_read: 0.20 }),
    ("opus-5", Price { input: 5.0, output: 25.0, cache_read: 0.50 }),
    ("opus-4-8", Price { input: 5.0, output: 25.0, cache_read: 0.50 }),
    ("opus-4-7", Price { input: 5.0, output: 25.0, cache_read: 0.50 }),
    ("opus-4-6", Price { input: 5.0, output: 25.0, cache_read: 0.50 }),
    ("sonnet-5-5", Price { input: 2.0, output: 10.0, cache_read: 0.10 }),
    ("sonnet-5", Price { input: 2.0, output: 10.0, cache_read: 0.20 }),
    ("sonnet-4-6", Price { input: 3.0, output: 15.0, cache_read: 0.30 }),
    ("haiku-5-5", Price { input: 0.10, output: 0.50, cache_read: 0.01 }),
    ("haiku-4-5", Price { input: 1.0, output: 5.0, cache_read: 0.10 }),
];

/// Writing to the cache costs more than plain input: 1.25x for the
/// 5-minute cache, 2x for the 1-hour one.
pub const CACHE_WRITE_5M: f64 = 1.25;
pub const CACHE_WRITE_1H: f64 = 2.0;

pub fn for_model(id: &str) -> Option<&'static Price> {
    let name = id.strip_prefix("claude-").unwrap_or(id);
    TABLE.iter().find(|(start, _)| name.starts_with(start)).map(|(_, price)| price)
}
