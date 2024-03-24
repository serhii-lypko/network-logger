/*
    -- Implementation Plan --

    1. Extract DNS records from network traffic
    2. Write to in-memory cache. When cache bound overflows -> init commit and clean cache.
    3. Implement commit logic
*/

extern crate pnet;

mod cache;
mod dns_parser;
mod packets_listener;

use tokio::sync::mpsc;

use std::sync::Arc;

use cache::Cache;
use packets_listener::PacketsListener;

// TODO: program performance, CPU and memory usage metrics

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(64);

    let listener = PacketsListener::new(Arc::new(tx));

    tokio::spawn(async move {
        listener.listen().await;
    });

    let mut cache = Cache::new(rx);
    // FIXME -> naming
    cache.process_messages().await;
}
