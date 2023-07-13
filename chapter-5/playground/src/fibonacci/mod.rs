pub struct Fibonacci {
    pub this: u32,
    pub next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let step = self.this + self.next;

        self.this = self.next;
        self.next = step;

        Some(self.this)
    }
}
