use tokio::sync::mpsc::Receiver;

use bytes::{BufMut, Bytes};

use anyhow::Result;

use log::debug;

// TODO
const BUFFER_LIMIT: usize = 2500;
const DELIMITER: u8 = 0x1E;

pub struct Commiter {
    rx: Receiver<Bytes>,
    buffer: Vec<u8>,
}

// TODO: tests

impl Commiter {
    pub fn new(rx: Receiver<Bytes>) -> Self {
        Commiter { rx, buffer: vec![] }
    }

    /*
        Algorithm

        Recieve a upd packet as bytes.
        Write it's content to buffer with delimiter
        At each update check buffer len. If it's become more N KB:
            1. create protobuf message of buffered data
            2. commit message
            3. release buffer
    */

    pub async fn process_messages(&mut self) -> Result<()> {
        while let Some(message) = self.rx.recv().await {
            let buff_size = self.write_to_buffer(message);

            if buff_size > BUFFER_LIMIT {
                debug!("Commit message...");

                self.commit().await?;
                self.buffer.clear();
            }
        }

        Ok(())
    }

    fn write_to_buffer(&mut self, message: Bytes) -> usize {
        self.buffer.put(message);
        self.buffer.put_u8(DELIMITER);

        self.buffer.len()
    }

    // TODO: create http service? (use rustify?)
    async fn commit(&self) -> Result<()> {
        // TODO: log when success

        todo!()
    }
}
