use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs;
use tokio::time::{sleep, Duration};

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    
    let filename = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            "hello.html"
        }
        (&Method::GET, "/sleep") => {
            sleep(Duration::from_secs(5)).await;
            "sleep.html"
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            "404.html"
        }
    };
    *response.body_mut() = Body::from(fs::read_to_string(filename).await.unwrap());
    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { 
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
