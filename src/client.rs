use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

use commiter::Commiter;
use packets_listener::PacketsListener;

use packet_transfer::packet_transfer_client::PacketTransferClient;
use packet_transfer::PacketData;

extern crate pnet;

pub mod packet_transfer {
    tonic::include_proto!("packet_transfer");
}

mod commiter;
mod dns_parser;
mod packets_listener;

use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Starting network logger client service");

    let (tx, rx) = mpsc::channel(64);

    let listener = PacketsListener::new(Arc::new(tx));

    tokio::spawn(async move { listener.listen().await });

    let grpc_client = PacketTransferClient::connect("http://0.0.0.0:50051").await?;

    let mut commiter = Commiter::new(grpc_client, rx);
    commiter.process_messages().await?;

    Ok(())
}
