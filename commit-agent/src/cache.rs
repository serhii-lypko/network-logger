use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

use log::debug;

// TODO: tests?

pub struct Cache {
    rx: Receiver<String>,
    storage: HashMap<String, u32>,
}

impl Cache {
    pub fn new(rx: Receiver<String>) -> Self {
        Cache {
            rx,
            storage: HashMap::new(),
        }
    }

    pub async fn process_messages(&mut self) {
        while let Some(message) = self.rx.recv().await {
            // algorithm: update cache, check it's size; if overflow -> commit and clear.

            // NOTE: tricky thing -> commit may take a while (API call). also it can fail.
            // so how to release the cache for new writes and not lost the data?

            debug!("Processing the message...");
        }
    }

    // TODO: error handling
    pub async fn commit(&self) {
        todo!()
    }
}
