use anyhow::Result;
use bytes::{BufMut, Bytes};
use chrono::Utc;
use tokio::sync::mpsc::Receiver;

use log::debug;

use crate::packets_transfer::packets_transfer_client::PacketsTransferClient;
use crate::PacketsData;

// TODO: tests?

const BUFFER_LIMIT: usize = 2500; // TODO appropriate size?
pub const UDP_MESSAGE_BYTES_DELIMITER: u8 = 0x1E;

pub struct Commiter {
    // NOTE: is a configuration deadlock safe?
    // https://github.com/hyperium/tonic/blob/master/examples/src/blocking/client.rs
    grpc_client: PacketsTransferClient<tonic::transport::Channel>,
    rx: Receiver<Bytes>,
    buffer: Vec<u8>,
}

impl Commiter {
    pub fn new(
        grpc_client: PacketsTransferClient<tonic::transport::Channel>,
        rx: Receiver<Bytes>,
    ) -> Self {
        Commiter {
            grpc_client,
            rx,
            buffer: Vec::with_capacity(BUFFER_LIMIT),
        }
    }

    pub async fn process_messages(&mut self) -> Result<()> {
        while let Some(message) = self.rx.recv().await {
            self.buffer.put(message);
            let buff_size = self.buffer.len();

            if buff_size > BUFFER_LIMIT {
                debug!("Commit buffered packets...");

                self.commit().await?;
                self.buffer.clear();
            } else {
                self.buffer.put_u8(UDP_MESSAGE_BYTES_DELIMITER);
            }
        }

        Ok(())
    }

    async fn commit(&mut self) -> Result<()> {
        let packets = tonic::Request::new(PacketsData {
            packets: self.buffer.clone(),
            timestamp: Utc::now().timestamp(),
        });

        self.grpc_client.transfer_packets(packets).await?;

        Ok(())
    }
}
