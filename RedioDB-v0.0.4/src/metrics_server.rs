// src/metrics_server.rs
use hyper::{Body, Response, Server, Request};
use hyper::service::{make_service_fn, service_fn};
use rediodb::monitoring::get_metrics;

async fn metrics_handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let metrics = get_metrics();
    Ok(Response::new(Body::from(metrics)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the metrics endpoint on port 9100 (default for Prometheus exporters is often 9100).
    let addr = ([0, 0, 0, 0], 9100).into();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(metrics_handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Metrics server running on http://{}", addr);

    server.await?;
    Ok(())
}
