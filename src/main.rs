// extern crate pnet;

// pub mod commiter;
// pub mod dns_parser;
// pub mod packets_listener;

// use std::sync::Arc;
// use tokio::sync::mpsc;

use anyhow::Result;

// use commiter::Commiter;
// use packets_listener::PacketsListener;

// use log::info;
// use std::env;

// pub mod packet_transfer {
//     tonic::include_proto!("packet_transfer");
// }

// Maket it work, make it right, make it fast

// NOTE: the program will not work as expected with VPN turned on for obvious reasons

// TODO: program performance, CPU and memory usage metrics

// NOTE: how to re-run program when laptop goes to sleep? -> use launchd? (a Launch Agent or Daemon)

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: load envs from config
    // env::set_var("RUST_LOG", "debug");
    // env_logger::init();

    // info!("Starting commit agent service");

    // let (tx, rx) = mpsc::channel(64);

    // let listener = PacketsListener::new(Arc::new(tx));

    // tokio::spawn(async move { listener.listen().await });

    // let mut commiter = Commiter::new(rx);
    // commiter.process_messages().await?;

    Ok(())
}
