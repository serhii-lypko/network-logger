use tokio::sync::mpsc::Receiver;

// TODO: tests?

// TODO: commitment logging?

pub struct Cache {
    // TODO: design cache data structure
    rx: Receiver<String>,
}

impl Cache {
    pub fn new(rx: Receiver<String>) -> Self {
        Cache { rx }
    }

    pub async fn process_messages(&mut self) {
        while let Some(message) = self.rx.recv().await {
            println!(
                "Hello fucking world! Message recieved god damn {:?}",
                message
            );
        }
    }

    pub fn commit() {
        todo!()
    }
}
