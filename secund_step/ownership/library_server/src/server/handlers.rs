use super::super::book::{Library};
use super::RequestBook;
use hyper::{Body, Request, Response, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use hyper::body::Buf;

async fn deserialize_type<T: for<'a> Deserialize<'a>>(req: Request<Body>) -> Result<T, Error> {
    let body = match hyper::body::aggregate(req.into_body()).await {
        Ok(body) => body,
        Err(err) => panic!("Aggregate failed: {}", err)
    };
    let json = match serde_json::from_reader(body.reader()) {
        Ok(json) => json,
        Err(err) => panic!("Aggregate failed: {}", err)
    };
    Ok(json)
}

fn to_response<T: Serialize>(res: T) -> Result<Response<Body>, Infallible> {
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

pub async fn req_to_type<T: for<'a> Deserialize<'a>>(req: Request<Body>) -> Option<T> {
    match deserialize_type::<T>(req).await {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

pub async fn root() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("🏛️ Welcome to the Library!".into())
        .unwrap())
}

pub async fn not_found(msg: String) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(msg.into())
        .unwrap())
}

pub async fn send_to_runner(req_book: RequestBook, tx: Sender<RequestBook>) -> Result<Response<Body>, Infallible> {
    tx.send(req_book.clone()).await.unwrap();
    to_response(req_book.id)
}

pub fn extract_id_from_url(uri: Uri) -> Option<Uuid> {
    let url = uri.to_string();
    let mut id: Option<Uuid> = None;

    for fragment in url.split("?").skip(1).flat_map(|s| s.split('&')) {
        let (key, value) = fragment.split_once('=').unwrap_or((fragment, ""));
        match key {
            "id" => {
                id = Uuid::parse_str(value.trim()).ok();
                break;
            }
            _ => {}
        }
    }
    id
}

pub async fn get_all_books() -> Result<Response<Body>, Infallible> {
    let library = Library::new();
    to_response(library.all_books())
}

pub async fn get_a_books(id: Uuid) -> Result<Response<Body>, Infallible> {
    let library = Library::new();
    if let Some(book) = library.get_book(id.to_string()) {
        to_response(book)
    } else {
        not_found("Book not exist".into()).await
    }
}
