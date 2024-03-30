use std::collections::HashMap;

use tonic::{transport::Server, Request, Response, Status};

use chrono::Utc;

use packets_transfer::packets_transfer_server::{PacketsTransfer, PacketsTransferServer};
use packets_transfer::{PacketsData, TransferAck};

use dns_parser::DNSParser;

pub mod packets_transfer {
    tonic::include_proto!("packet_transfer");
}

mod dns_parser;

// TODO: it's important to make sure the cloud DB storage won't be exceeded too fast -> write metrics

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

// TODO: better naming?
struct NetworkLogRecord {
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

#[derive(Debug, Default)]
pub struct PacketTransferService {
    foo: String, // TODO: database instance instead?
}

impl PacketTransferService {
    pub fn new(foo: String) -> Self {
        PacketTransferService { foo }
    }
}

#[tonic::async_trait]
impl PacketsTransfer for PacketTransferService {
    async fn transfer_packets(
        &self,
        packets_data_request: Request<PacketsData>,
    ) -> Result<Response<TransferAck>, Status> {
        let packets_data = packets_data_request.get_ref().clone();

        let timestamp = packets_data.timestamp;
        let dns_records = DNSParser::parse_packets(packets_data.packets);

        let network_log_record: NetworkLogRecord = (timestamp, dns_records).into();

        // TODO: write network_log_record to DB (mongo?)

        let reply = packets_transfer::TransferAck {
            message: "Acknowledge the packet transfer".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse::<std::net::SocketAddr>()?;
    let service = PacketTransferService::new("hello".to_string());

    Server::builder()
        .add_service(PacketsTransferServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
