use clap::Parser;
use libp2p_broadcast::protocol::Message;
use libp2p_broadcast::{Broadcast, BroadcastConfig, Topic /*, BroadcastEvent*/};
use log::debug;
use std::sync::Arc;

fn get_git_head_short_hash() -> String {
    let repo = match git2::Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return "my-default-topic".to_string(), // Fallback if not a git repo
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return "my-default-topic".to_string(), // Fallback if HEAD is detached or invalid
    };

    let oid = match head.target() {
        Some(oid) => oid,
        None => return "my-default-topic".to_string(), // Fallback if no target OID
    };

    let commit = match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(_) => return "my-default-topic".to_string(), // Fallback if commit not found
    };

    commit.as_object().short_id().unwrap().as_str().unwrap().to_string()
}

/// Simple program to broadcast messages
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Topic to subscribe to and broadcast on
    #[arg(short, long, default_value_t = get_git_head_short_hash())]
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
        Message::Subscribe(topic.clone()),
        Message::Unsubscribe(topic.clone()),
        Message::Broadcast(topic, Arc::new(*b"content")),
    ];
    for msg in &msgs {
        println!("msg: {:?}", msg);
        let msg2 = Message::from_bytes(&msg.to_bytes()).unwrap();
        assert_eq!(msg, &msg2);
    }
}
