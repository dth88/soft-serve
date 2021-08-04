use std::io::Error;
use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(soft_serve))
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


async fn soft_serve(_req: Request<Body>)
-> Result<Response<Body>, Error> {
    let static_ = hyper_staticfile::Static::new("static/");
    let response_future = static_.serve(_req).await;
    response_future
}
