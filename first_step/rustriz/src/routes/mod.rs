mod handlers;

use handlers::{multiply, not_found, root, transpose};
use hyper::Method;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn routing(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Request");
    dbg!(&_req);
    match (_req.method(), _req.uri().path()) {
        (&Method::POST, "/mul") => multiply(_req).await,
        (&Method::POST, "/T") => transpose(_req).await,
        (&Method::GET, "/") => root().await,
        _ => not_found(_req.uri().path()).await,
    }
}
