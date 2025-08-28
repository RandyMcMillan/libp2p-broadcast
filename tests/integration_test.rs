use libp2p_broadcast::{Broadcast, BroadcastConfig, Topic};

#[test]
fn test_new_broadcast_instance() {
    let config = BroadcastConfig::default();
    let _broadcast = Broadcast::new(config);
    // Add assertions here to verify the initial state of the Broadcast instance
    // For example, check if subscriptions are empty
    // assert!(_broadcast.subscribed().next().is_none());
}

#[test]
fn test_topic_creation() {
    let topic_name = b"test_topic";
    let topic = Topic::new(topic_name);
    assert_eq!(topic.as_ref(), topic_name);
}
