use std::convert::Infallible;
use std::str::FromStr;

use crate::domain::job::WasmJob;

use hyper::{body::Buf, Body, Request, Response, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub async fn deserialize_type<T: for<'a> Deserialize<'a>>(req: Request<Body>) -> Result<T, Error> {
    let body = match hyper::body::aggregate(req.into_body()).await {
        Ok(body) => body,
        Err(err) => panic!("Aggregate failed: {}", err),
    };
    let json = match serde_json::from_reader(body.reader()) {
        Ok(json) => json,
        Err(err) => panic!("Aggregate failed: {}", err),
    };
    Ok(json)
}

pub async fn req_to_type<T: for<'a> Deserialize<'a>>(req: Request<Body>) -> Option<T> {
    match deserialize_type::<T>(req).await {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

pub fn invalid_wasm_response() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid Wasm".into())
        .unwrap())
}

pub fn invalid_wasm_id_response() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid Wasm ID".into())
        .unwrap())
}

pub fn to_response<T: Serialize>(res: T) -> Result<Response<Body>, Infallible> {
    let res = match serde_json::to_string(&res) {
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

pub async fn not_found(msg: String) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(msg.into())
        .unwrap())
}

pub async fn send_to_runner(
    wasm_job: WasmJob,
    tx: Sender<WasmJob>,
) -> Result<Response<Body>, Infallible> {
    tx.send(wasm_job.clone()).await.unwrap();
    to_response(wasm_job.cloned_id())
}

pub fn extract_id_from_url(uri: Uri) -> Option<String> {
    uri.to_string()
        .split('?')
        .skip(1)
        .flat_map(|s| s.split('&'))
        .filter_map(|fragment| {
            let (key, value) = fragment.split_once('=').unwrap_or((fragment, ""));
            if key == "id" {
                Uuid::from_str(value.trim())
                    .ok()
                    .map(|uuid| uuid.to_string())
            } else {
                None
            }
        })
        .next()
}

pub fn clear_terminal_with(string: &str) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", string);
}
