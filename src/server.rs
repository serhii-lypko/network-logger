use tonic::{transport::Server, Request, Response, Status};

use packets_transfer::packets_transfer_server::{PacketsTransfer, PacketsTransferServer};
use packets_transfer::{PacketsData, TransferAck};

use dns_parser::DNSParser;

pub mod packets_transfer {
    tonic::include_proto!("packet_transfer");
}

mod dns_parser;

#[derive(Debug, Default)]
pub struct PacketTransferService {
    foo: String,
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
        // let packets: Vec<&[u8]> = packets_data_request.split(|&byte| byte == 0x1E).collect();

        let timestamp = packets_data.timestamp;
        let dns_records = DNSParser::parse_packets(packets_data.packets);

        dbg!(dns_records);
        print!("---------------------------");

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
