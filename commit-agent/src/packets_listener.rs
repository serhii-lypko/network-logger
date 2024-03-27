use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use tokio::task;

use bytes::Bytes;

use pnet::datalink::{self, Channel};
use pnet::packet::{ethernet, ip, ipv4, udp, Packet};

use anyhow::Result;

use log::error;

const ETHERNET_INTERFACE: &str = "en0";
const DNS_PORT: u16 = 53;

pub struct PacketsListener {
    tx: Arc<Sender<Bytes>>,
}

impl PacketsListener {
    pub fn new(tx: Arc<Sender<Bytes>>) -> Self {
        PacketsListener { tx }
    }

    pub async fn listen(&self) -> Result<()> {
        let tx = self.tx.clone();

        let worker = task::spawn_blocking(move || {
            let interfaces = datalink::interfaces();

            let interface = interfaces
                .into_iter()
                .find(|iface| iface.name == ETHERNET_INTERFACE)
                .expect("Error finding interface.");

            let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("Error creating datalink channel: {}", e),
            };

            loop {
                match rx.next() {
                    Ok(packet) => {
                        let ethernet = match ethernet::EthernetPacket::new(packet) {
                            Some(packet) => packet,
                            None => continue, // Skip packets that can't be parsed
                        };

                        if let Some(packet) = ipv4::Ipv4Packet::new(ethernet.payload()) {
                            if packet.get_next_level_protocol() == ip::IpNextHeaderProtocols::Udp {
                                if let Some(udp_packet) = udp::UdpPacket::new(packet.payload()) {
                                    if udp_packet.get_destination() == DNS_PORT
                                        || udp_packet.get_source() == DNS_PORT
                                    {
                                        let payload = Bytes::copy_from_slice(udp_packet.payload());

                                        if let Err(err) = tx.try_send(payload) {
                                            error!("Failed to send packet to commiter: {}", err);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        panic!("An error occurred while reading: {}", e);
                    }
                }
            }
        });

        worker.await?;

        Ok(())
    }
}
