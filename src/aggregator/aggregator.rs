pub struct ResultAggregator {
    results: Vec<String>,
}

impl ResultAggregator {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add(&mut self, result: String) {
        self.results.push(result);
    }

    pub fn merge(&self) -> String {
        self.results.join("\n")
    }

    pub fn clear(&mut self) {
        self.results.clear();
    }
}
