pub struct Source(String);

impl Source {
    pub fn new(source: String) -> Self {
        Self(source)
    }

    pub fn source(&self) -> &str {
        &self.0
    }
}
