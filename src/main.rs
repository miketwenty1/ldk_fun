use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::lightning_invoice::Invoice;
use ldk_node::Builder;
use std::str::FromStr;

fn main() {
    let mut builder = Builder::new();
    builder.set_network(Network::Testnet);
    builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());
    builder.set_gossip_source_rgs(
        "https://rapidsync.lightningdevkit.org/testnet/snapshot".to_string(),
    );

    let node = builder.build().unwrap();

    node.start().unwrap();

    let funding_address = node.new_onchain_address();

    // .. fund address ..

    let node_id = PublicKey::from_str("NODE_ID").unwrap();
    let node_addr = SocketAddress::from_str("IP_ADDR:PORT").unwrap();
    node.connect_open_channel(node_id, node_addr, 10000, None, None, false)
        .unwrap();

    let event = node.wait_next_event();
    println!("EVENT: {:?}", event);
    node.event_handled();

    let invoice = Invoice::from_str("INVOICE_STR").unwrap();
    node.send_payment(&invoice).unwrap();

    node.stop().unwrap();
}
