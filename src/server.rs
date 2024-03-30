use tonic::{transport::Server, Request, Response, Status};

use packets_transfer::packets_transfer_server::{PacketsTransfer, PacketsTransferServer};
use packets_transfer::{PacketsData, TransferAck};

pub mod packets_transfer {
    tonic::include_proto!("packet_transfer");
}

#[derive(Debug, Default)]
pub struct PacketTransferService {}

#[tonic::async_trait]
impl PacketsTransfer for PacketTransferService {
    async fn transfer_packets(
        &self,
        request: Request<PacketsData>,
    ) -> Result<Response<TransferAck>, Status> {
        println!("Got packets: {:?}", request);
        println!("-------------");

        let reply = packets_transfer::TransferAck {
            message: "Acknowledge the packet transfer".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse::<std::net::SocketAddr>()?;
    let service = PacketTransferService::default();

    Server::builder()
        .add_service(PacketsTransferServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
