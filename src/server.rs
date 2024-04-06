use std::collections::HashMap;
use std::env;

use tonic::{transport::Server, Request, Response, Status};

use anyhow::Result;
use log::{debug, error, info};

use packets_transfer::packets_transfer_server::{PacketsTransfer, PacketsTransferServer};
use packets_transfer::{PacketsData, TransferAck};

use db_client::DbClient;
use dns_parser::DNSParser;

pub mod packets_transfer {
    tonic::include_proto!("packet_transfer");
}

mod db_client;
mod dns_parser;

/* ------------- ------------- ------------- ------------- -------------
Database (document?) schema

{
    timstamp: 123456,
    dns_log: {
        "google.com": 12,
        "facebook.com": 3,
        "api.openai.com": 6
    }
}

------------- ------------- ------------- ------------- ------------- */

#[derive(Debug)]
pub struct NetworkLogRecord {
    timestamp: i64,
    network_log: HashMap<String, u32>,
}

impl From<(i64, Vec<String>)> for NetworkLogRecord {
    fn from((timestamp, dns_list): (i64, Vec<String>)) -> Self {
        // TODO
        let network_log = HashMap::new();

        NetworkLogRecord {
            timestamp,
            network_log,
        }
    }
}

#[derive(Debug)]
pub struct PacketTransferService {
    db_client: DbClient,
}

impl PacketTransferService {
    pub fn new(db_client: DbClient) -> Self {
        PacketTransferService { db_client }
    }
}

#[tonic::async_trait]
impl PacketsTransfer for PacketTransferService {
    async fn transfer_packets(
        &self,
        packets_data_request: Request<PacketsData>,
    ) -> Result<Response<TransferAck>, Status> {
        let reply = packets_transfer::TransferAck {
            message: "Acknowledge the packet transfer".to_string(),
        };

        let packets_data = packets_data_request.get_ref().clone();

        let timestamp = packets_data.timestamp;
        let dns_records = DNSParser::parse_packets(packets_data.packets);
        let network_log_record: NetworkLogRecord = (timestamp, dns_records).into();

        if let Err(err) = self.db_client.write(network_log_record).await {
            error!("Error writing network log to DBL: {:?}", err);
        }

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Staring gRPC server");

    let grpc_socket_addr = "0.0.0.0:50051".parse::<std::net::SocketAddr>()?;
    let db_client = DbClient::new().await?;
    let grpc = PacketTransferService::new(db_client);

    Server::builder()
        .add_service(PacketsTransferServer::new(grpc))
        .serve(grpc_socket_addr)
        .await?;

    Ok(())
}
