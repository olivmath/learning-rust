use super::book::Library;
use super::server::{RequestBook, RequestType};
use tokio::sync::mpsc::Receiver;

pub struct Runner {
    rx: Receiver<RequestBook>,
    library: Library,
}

impl Runner {
    pub fn new(rx: Receiver<RequestBook>) -> Self {
        Runner {
            rx,
            library: Library::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(req_book) = self.rx.recv().await {
            match req_book.req_type {
                RequestType::SAVE => {
                    self.library
                        .add_book(req_book.cloned_id(), req_book.cloned_book());
                }
                RequestType::MODIFY => {
                    self.library
                        .modify_book(req_book.cloned_id(), req_book.cloned_book());
                }
                RequestType::DELETE => {
                    self.library.remove_book(req_book.cloned_id());
                }
            }
        }
    }
}
