use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

use commiter::Commiter;
use packets_listener::PacketsListener;

use packets_transfer::packets_transfer_client::PacketsTransferClient;
use packets_transfer::PacketsData;

extern crate pnet;

pub mod packets_transfer {
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

    // TODO: env for a port?
    let grpc_client = PacketsTransferClient::connect("http://0.0.0.0:50051").await?;

    let mut commiter = Commiter::new(grpc_client, rx);
    commiter.process_messages().await?;

    Ok(())
}
