use libp2p_broadcast::protocol::Message;
use libp2p_broadcast::{Broadcast, BroadcastConfig, Topic /*, BroadcastEvent*/};
use log::debug;
//use futures::future::BoxFuture;
//use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
//use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
//use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;

fn main() {
    debug!("Hello from libp2p-broadcast binary!");

    let mut topic = Topic::new(b"my-topic-1");

    let msgs = [
        Message::Broadcast(Topic::new(b""), Arc::new(*b"")),
        Message::Subscribe(topic),
        Message::Unknown(),
        Message::Unsubscribe(topic),
        Message::Broadcast(topic, Arc::new(*b"content")),
    ];
    for msg in &msgs {
        println!("msg: {:?}", msg);
        let msg2 = Message::from_bytes(&msg.to_bytes()).unwrap();
        //assert_eq!(msg, &msg2);
    }

    topic = Topic::new(b"my-topic-2");
    let config = BroadcastConfig::default();
    let mut broadcast = Broadcast::new(config);
    broadcast.subscribe(topic.clone());
    println!("Subscribed to topic: {:?}", topic);
    broadcast.broadcast(&topic, Arc::new(*b"my-topic-2 content!"));
    let msgs = [
        Message::Broadcast(Topic::new(b""), Arc::new(*b"")),
        Message::Subscribe(Topic::new(b"")),
        Message::Subscribe(topic),
        Message::Unsubscribe(topic),
        Message::Broadcast(topic, Arc::new(*b"content")),
    ];
    for msg in &msgs {
        println!("msg: {:?}", msg);
        let msg2 = Message::from_bytes(&msg.to_bytes()).unwrap();
        assert_eq!(msg, &msg2);
    }
}
