use libp2p_broadcast::{Broadcast, BroadcastConfig, Topic, BroadcastEvent};

fn main() {
    println!("Hello from libp2p-broadcast binary!");

    let config = BroadcastConfig::default();
    let mut broadcast = Broadcast::new(config);

    let topic = Topic::new(b"my-topic");
    broadcast.subscribe(topic.clone());

    println!("Subscribed to topic: {:?}", topic);
}