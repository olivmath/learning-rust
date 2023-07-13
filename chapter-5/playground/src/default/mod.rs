use core::fmt;

#[derive(Debug)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self("John Galt".to_string())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My name is `{}`", self.0)
    }
}
