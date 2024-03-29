use anyhow::Result;
use bytes::{BufMut, Bytes};
use chrono::Utc;
use tokio::sync::mpsc::Receiver;

use log::debug;

use crate::packet_transfer::packet_transfer_client::PacketTransferClient;
use crate::PacketData;

// TODO
const BUFFER_LIMIT: usize = 2500;
const DELIMITER: u8 = 0x1E;

pub struct Commiter {
    // NOTE: is configuration deadlock safe?
    grpc_client: PacketTransferClient<tonic::transport::Channel>,
    rx: Receiver<Bytes>,
    buffer: Vec<u8>,
}

// TODO: tests?

impl Commiter {
    pub fn new(
        grpc_client: PacketTransferClient<tonic::transport::Channel>,
        rx: Receiver<Bytes>,
    ) -> Self {
        Commiter {
            grpc_client,
            rx,
            buffer: vec![],
        }
    }

    pub async fn process_messages(&mut self) -> Result<()> {
        while let Some(message) = self.rx.recv().await {

            // -------

            // let buff_size = self.write_to_buffer(message);

            // if buff_size > BUFFER_LIMIT {
            //     debug!("Commit message...");

            //     self.commit().await?;
            //     self.buffer.clear();
            // }
        }

        Ok(())
    }

    fn write_to_buffer(&mut self, message: Bytes) -> usize {
        self.buffer.put(message);
        self.buffer.put_u8(DELIMITER);

        self.buffer.len()
    }

    async fn commit(&mut self, message: Bytes) -> Result<()> {
        let request = tonic::Request::new(PacketData {
            packets: message.to_vec(),
            timestamp: Utc::now().timestamp(),
        });

        let _response = self.grpc_client.transfer_packets(request).await?;

        todo!()
    }
}
