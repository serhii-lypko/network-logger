use packet_transfer::packet_transfer_client::PacketTransferClient;
use packet_transfer::PacketData;

use chrono::Utc;

pub mod packet_transfer {
    tonic::include_proto!("packet_transfer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PacketTransferClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(PacketData {
        packets: vec![],
        timestamp: Utc::now().timestamp(),
    });

    let response = client.transfer_packets(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
