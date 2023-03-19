use crate::domain::job::{JobType, WasmJob};

use super::super::domain::Storage;
use super::request::WasmRequest;
use super::utils::{
    extract_id_from_url, invalid_wasm_id_response, invalid_wasm_response, not_found, req_to_type,
    send_to_runner, to_response,
};
use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub async fn handler_routes(
    tx: Sender<WasmJob>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            if let Some(wasm_req) = req_to_type::<WasmRequest>(req).await {
                let id = Uuid::new_v4().to_string();
                let wasm_job =
                    WasmJob::new(wasm_req.wasm, id, wasm_req.run_function, JobType::SAVE);
                send_to_runner(wasm_job, tx.clone()).await
            } else {
                invalid_wasm_response()
            }
        }
        (&Method::PUT, path) if path.starts_with("/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                if let Some(wasm_req) = req_to_type::<WasmRequest>(req).await {
                    let wasm_job =
                        WasmJob::new(wasm_req.wasm, id, wasm_req.run_function, JobType::MODIFY);
                    send_to_runner(wasm_job, tx.clone()).await
                } else {
                    invalid_wasm_response()
                }
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::DELETE, path) if path.starts_with("/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                let mut wasm_job = WasmJob::default();
                wasm_job.set_id(id);
                send_to_runner(wasm_job, tx.clone()).await
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::GET, "/run/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                dbg!(id);
                // enviar pedido de execusao para o canal espera a resposta
                // worker quando ouvir o pedido executa e envia a resposta
                // quando ouvir a resposta devolve para o client
                to_response("rodou function".to_string())
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::GET, "/wasm/") => {
            if let Some(id) = extract_id_from_url(req.uri().clone()) {
                let storage = Storage::new();
                if let Some(wasm) = storage.get_wasm(id.to_string()) {
                    dbg!("not running");
                    to_response(wasm)
                } else {
                    not_found("wasm not exist".into()).await
                }
            } else {
                invalid_wasm_id_response()
            }
        }
        (&Method::GET, "/") => {
            let storage = Storage::new();
            to_response(storage.all_wasms())
        }
        _ => not_found("Page not found".into()).await,
    }
}
