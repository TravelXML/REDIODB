use rediodb::pubsub::PubSub;

#[tokio::test]
async fn test_pubsub() {
    // Create a new PubSub instance.
    let pubsub = PubSub::new();

    // Create two subscribers to the broadcast channel.
    let mut subscriber1 = pubsub.subscribe();
    let mut subscriber2 = pubsub.subscribe();

    // Define the message to publish.
    let message = "Hello, subscribers!".to_string();

    // Publish the message.
    pubsub.publish(message.clone());

    // Both subscribers should receive the message.
    let received1 = subscriber1.recv().await.expect("Subscriber1 did not receive message");
    let received2 = subscriber2.recv().await.expect("Subscriber2 did not receive message");

    // Assert that both received messages match the published message.
    assert_eq!(received1, message);
    assert_eq!(received2, message);
}