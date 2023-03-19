use crate::domain::matrix::{Matrices, Matrix3x3};
use hyper::StatusCode;
use hyper::{Body, Request, Response};
use serde::Deserialize;
use serde_json::Error;
use std::convert::Infallible;

async fn req_to_type<T: for<'a> Deserialize<'a>>(req: Request<Body>) -> Result<T, Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str)
}

pub async fn multiply(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let matrices: Matrices = match req_to_type(req).await {
        Ok(m) => m,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid Request Body".into())
                .unwrap())
        }
    };

    let res = match serde_json::to_string(&matrices.a.mul(matrices.b)) {
        Ok(s) => s,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Internal Server Error: {:?}", e).into())
                .unwrap())
        }
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(res.into())
        .unwrap())
}

pub async fn transpose(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let matrix: Matrix3x3 = match req_to_type(req).await {
        Ok(m) => m,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid Request Body".into())
                .unwrap())
        }
    };

    let res = match serde_json::to_string(&matrix.transpose()) {
        Ok(s) => s,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Internal Server Error: {:?}", e).into())
                .unwrap())
        }
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(res.into())
        .unwrap())
}

pub async fn root() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("Rustriz say: Hello 🧮!".into())
        .unwrap())
}

pub async fn not_found(url: &str) -> Result<Response<Body>, Infallible> {
    dbg!(url);
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("404 Not Found".into())
        .unwrap())
}
