use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use tokio::task;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

use crate::dns_parser::DNSParser;

const ETHERNET_INTERFACE: &str = "en0";
const DNS_PORT: u16 = 53;

pub struct PacketsListener {
    tx: Arc<Sender<String>>,
}

impl PacketsListener {
    pub fn new(tx: Arc<Sender<String>>) -> Self {
        PacketsListener { tx }
    }

    // TODO: error handling
    pub async fn listen(&self) {
        let tx = self.tx.clone();

        let worker = task::spawn_blocking(move || {
            let interfaces = datalink::interfaces();

            let interface = interfaces
                .into_iter()
                .find(|iface| iface.name == ETHERNET_INTERFACE)
                .expect("Error finding interface.");

            let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("Error creating datalink channel: {}", e),
            };

            loop {
                match rx.next() {
                    Ok(packet) => {
                        let ethernet = EthernetPacket::new(packet).unwrap();

                        if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
                            if packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                                if let Some(udp_packet) = UdpPacket::new(packet.payload()) {
                                    if udp_packet.get_destination() == DNS_PORT
                                        || udp_packet.get_source() == DNS_PORT
                                    {
                                        let payload = udp_packet.payload();
                                        // let dns = DNSParser::parse_packet(payload);

                                        // TODO
                                        match tx
                                            .try_send(String::from("hello world fucking message!"))
                                        {
                                            Ok(_) => (),
                                            Err(e) => {
                                                eprintln!("Failed to send message: {}", e);
                                            }
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

        worker.await.unwrap();
    }
}
