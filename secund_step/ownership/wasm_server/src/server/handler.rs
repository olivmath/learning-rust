use super::super::wasm::{Storage, Wasm};

use super::request::RequestType;
use super::utils::{
    extract_id_from_url, invalid_wasm_id_response, invalid_wasm_response, not_found, req_to_type,
    send_to_runner, to_response,
};
use super::RequestWasm;
use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub async fn handler_routes(
    tx: Sender<RequestWasm>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            if let Some(wasm) = req_to_type::<Wasm>(req).await {
                let id = Uuid::new_v4().to_string();
                let req_wasm = RequestWasm::new(id, wasm, RequestType::SAVE);
                send_to_runner(req_wasm, tx.clone()).await
            } else {
                invalid_wasm_response()
            }
        }
        (&Method::PUT, path) if path.starts_with("/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                if let Some(wasm) = req_to_type::<Wasm>(req).await {
                    let req_wasm = RequestWasm::new(id.to_string(), wasm, RequestType::MODIFY);
                    send_to_runner(req_wasm, tx.clone()).await
                } else {
                    invalid_wasm_response()
                }
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::DELETE, path) if path.starts_with("/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                let req_wasm = RequestWasm::from_request(id.to_string(), RequestType::DELETE);
                send_to_runner(req_wasm, tx.clone()).await
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::GET, "/") => {
            let storage = Storage::new();
            to_response(storage.all_wasms())
        }
        (&Method::GET, path) if path.starts_with("/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                let storage = Storage::new();
                if let Some(wasm) = storage.get_wasm(id.to_string()) {
                    to_response(wasm)
                } else {
                    not_found("wasm not exist".into()).await
                }
            } else {
                invalid_wasm_id_response()
            }
        }
        _ => not_found("Page not found".into()).await,
    }
}
