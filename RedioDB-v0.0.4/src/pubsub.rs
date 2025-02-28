// src/pubsub.rs
//
// A simple Pub/Sub system using Tokio broadcast channels.
use tokio::sync::broadcast;

/// PubSub structure encapsulating a broadcast sender.
pub struct PubSub {
    sender: broadcast::Sender<String>,
}

impl PubSub {
    /// Creates a new PubSub instance with a channel capacity of 100.
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(100);
        PubSub { sender }
    }

    /// Publishes a message to all subscribers.
    pub fn publish(&self, message: String) {
        let _ = self.sender.send(message);
    }

    /// Returns a new subscriber to the broadcast channel.
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
}
