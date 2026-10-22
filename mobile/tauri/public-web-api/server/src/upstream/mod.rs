//! Clients for the public APIs this service scrapes. Plain `reqwest`
//! against documented JSON endpoints — no vendor SDKs.

mod hn;

pub use hn::{HackerNews, Item, UpstreamError};
