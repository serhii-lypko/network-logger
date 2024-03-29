use tonic::{transport::Server, Request, Response, Status};

use packet_transfer::packet_transfer_server::{PacketTransfer, PacketTransferServer};
use packet_transfer::{PacketData, TransferAck};

pub mod packet_transfer {
    tonic::include_proto!("packet_transfer");
}

#[derive(Debug, Default)]
pub struct PacketTransferService {}

#[tonic::async_trait]
impl PacketTransfer for PacketTransferService {
    async fn transfer_packets(
        &self,
        request: Request<PacketData>,
    ) -> Result<Response<TransferAck>, Status> {
        println!("Got a request: {:?}", request);

        let reply = packet_transfer::TransferAck {
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
        .add_service(PacketTransferServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
