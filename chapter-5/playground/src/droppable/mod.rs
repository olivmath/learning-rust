pub struct Droppable {
    pub data: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}
