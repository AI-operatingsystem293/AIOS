#[derive(Debug, Clone)]
pub struct TaskEdge {
    pub parent: u64,
    pub child: u64,
}

pub struct TaskGraph {
    edges: Vec<TaskEdge>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, parent: u64, child: u64) {
        self.edges.push(TaskEdge { parent, child });
    }

    pub fn children(&self, parent: u64) -> Vec<u64> {
        self.edges
            .iter()
            .filter(|e| e.parent == parent)
            .map(|e| e.child)
            .collect()
    }
}
