mod handlers;

use handlers::{
    extract_id_from_url, get_a_books, get_all_books, not_found, req_to_type, root, send_to_runner,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::book::Book;

fn clear_terminal_with(string: &str) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", string);
}

#[derive(Serialize, Clone)]
pub struct ResponseBook {
    pub title: String,
    pub year: u16,
    pub id: String,
}

impl ResponseBook {
    fn new(title: String, year: u16, id: String) -> Self {
        ResponseBook { title, year, id }
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum RequestType {
    SAVE,
    MODIFY,
    DELETE,
}

#[derive(Clone, Serialize, Debug)]
pub struct RequestBook {
    pub req_type: RequestType,
    pub id: String,
    pub book: Book,
}

impl RequestBook {
    pub fn new(id: String, book: Book, req_type: RequestType) -> Self {
        RequestBook { req_type, id, book }
    }

    pub fn cloned_book(&self) -> Book {
        self.book.clone()
    }

    pub fn cloned_id(&self) -> String {
        self.id.clone()
    }

    pub fn from_request(id: String, req_type: RequestType) -> Self {
        RequestBook {
            req_type,
            id,
            book: Book::default(),
        }
    }
}

pub struct MyServer {
    addr: SocketAddr,
}

impl MyServer {
    pub fn new(addr: SocketAddr) -> MyServer {
        MyServer { addr }
    }

    pub async fn run(self, tx: Sender<RequestBook>) -> Result<(), Infallible> {
        clear_terminal_with("🔗 Binding server in http://127.0.0.1:3000");

        let make_service = make_service_fn(move |_connection| {
            let _tx = tx.clone();
            async { Ok::<_, Infallible>(service_fn(move |req| Self::router(_tx.clone(), req))) }
        });

        let server = Server::bind(&self.addr).serve(make_service);

        println!("🚀 Run Library Server 📚");

        if let Err(e) = server.await {
            eprintln!("{}", e);
        }
        Ok(())
    }

    pub async fn router(
        tx: Sender<RequestBook>,
        req: Request<Body>,
    ) -> Result<Response<Body>, Infallible> {
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/book") => {
                if let Some(book) = req_to_type::<Book>(req).await {
                    let id = Uuid::new_v4().to_string();
                    let req_book = RequestBook::new(id, book, RequestType::SAVE);
                    send_to_runner(req_book, tx.clone()).await
                } else {
                    invalid_book_response()
                }
            }
            (&Method::PUT, path) if path.starts_with("/book/") => {
                if let Some(id) = extract_id_from_url(req.uri().clone()) {
                    if let Some(book) = req_to_type::<Book>(req).await {
                        let req_book = RequestBook::new(id.to_string(), book, RequestType::MODIFY);
                        send_to_runner(req_book, tx.clone()).await
                    } else {
                        invalid_book_response()
                    }
                } else {
                    invalid_book_id_response()
                }
            }
            (&Method::DELETE, path) if path.starts_with("/book/") => {
                if let Some(id) = extract_id_from_url(req.uri().clone()) {
                    let req_book = RequestBook::from_request(id.to_string(), RequestType::DELETE);
                    send_to_runner(req_book, tx.clone()).await
                } else {
                    invalid_book_id_response()
                }
            }
            (&Method::GET, "/book") => get_all_books().await,
            (&Method::GET, path) if path.starts_with("/book/") => {
                if let Some(id) = extract_id_from_url(req.uri().clone()) {
                    get_a_books(id).await
                } else {
                    invalid_book_id_response()
                }
            }
            (&Method::GET, "/") => root().await,
            _ => not_found("Page not found".into()).await,
        }
    }
}
fn invalid_book_response() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid Book".into())
        .unwrap())
}

fn invalid_book_id_response() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid Book ID".into())
        .unwrap())
}
