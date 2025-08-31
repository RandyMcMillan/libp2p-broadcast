use clap::Parser;
use libp2p_broadcast::protocol::Message;
use libp2p_broadcast::{Broadcast, BroadcastConfig, Topic /*, BroadcastEvent*/};
use log::debug;
//use futures::future::BoxFuture;
//use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
//use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
//use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;

/// Simple program to broadcast messages
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Topic to subscribe to and broadcast on
    #[arg(short, long, default_value = "my-default-topic")]
    topic: String,
}

fn main() {
    let cli = Cli::parse();

    debug!("Hello from libp2p-broadcast binary!");

    let topic = Topic::new(cli.topic.as_bytes());

    let msgs = [
        Message::Broadcast(Topic::new(b""), Arc::new(*b"")),
        Message::Subscribe(topic.clone()),
        Message::Unsubscribe(topic.clone()),
        Message::Broadcast(topic.clone(), Arc::new(*b"content")),
    ];
    for msg in &msgs {
        println!("msg: {:?}", msg);
        let _msg2 = Message::from_bytes(&msg.to_bytes()).unwrap();
    }

    let config = BroadcastConfig::default();
    let mut broadcast = Broadcast::new(config);
    broadcast.subscribe(topic.clone());
    println!("Subscribed to topic: {:?}", topic);
    broadcast.broadcast(&topic, Arc::new(*b"my-topic-2 content!"));
    let msgs = [
        Message::Broadcast(Topic::new(b""), Arc::new(*b"")),
        Message::Subscribe(Topic::new(b"")),
        Message::Subscribe(&topic),
        Message::Unsubscribe(&topic),
        Message::Broadcast(topic, Arc::new(*b"content")),
    ];
    for msg in &msgs {
        println!("msg: {:?}", msg);
        let msg2 = Message::from_bytes(&msg.to_bytes()).unwrap();
        assert_eq!(msg, &msg2);
    }
}
