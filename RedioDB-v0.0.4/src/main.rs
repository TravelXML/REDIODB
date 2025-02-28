// src/main.rs
//

use tonic::transport::Server;
use rediodb::server::rediodb_server::rediodb_server::RediodbServer;
use rediodb::server::my_service::MyService;
use env_logger;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging.
    env_logger::init();

    // Read server address from environment variable REDIO_ADDRESS or default.
    let addr_str = env::var("REDIO_ADDRESS").unwrap_or_else(|_| "0.0.0.0:50051".to_string());
    let addr = addr_str.parse()?;
    
    let service = MyService::default();

    println!("Starting REDIODB server on {}", addr);

    Server::builder()
        .add_service(RediodbServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
