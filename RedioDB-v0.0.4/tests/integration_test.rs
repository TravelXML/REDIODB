use tokio::time::{sleep, Duration};
use rediodb::server::rediodb_server::edgedb_client::EdgedbClient;
use rediodb::server::rediodb_server::{SetRequest, KeyRequest, ResponseMessage, ValueResponse};
use tonic::Request; // Import tonic::Request

#[tokio::test]
async fn test_set_get_expire() {
    // Connect to the gRPC server.
    let mut client = EdgedbClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Failed to connect to gRPC server");

    // Test SET with a TTL of 3 seconds.
    let set_req = Request::new(SetRequest {
        key: "integrationKey".into(),
        value: "Integration Test".into(),
        ttl: 3,
    });
    let set_resp: ResponseMessage = client.set(set_req).await.unwrap().into_inner();
    assert_eq!(set_resp.status, "success");

    // Immediately GET the key.
    let get_req = Request::new(KeyRequest {
        key: "integrationKey".into(),
    });
    let get_resp: ValueResponse = client.get(get_req).await.unwrap().into_inner();
    assert_eq!(get_resp.value, "Integration Test");

    // Wait for 4 seconds so the key expires.
    sleep(Duration::from_secs(4)).await;

    // GET the key again; it should have expired.
    let get_req2 = Request::new(KeyRequest {
        key: "integrationKey".into(),
    });
    let get_resp2: ValueResponse = client.get(get_req2).await.unwrap().into_inner();
    assert_eq!(get_resp2.value, "");
}