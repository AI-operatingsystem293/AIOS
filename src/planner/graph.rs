#[derive(Clone, Debug)]
pub struct TaskNode {
    pub id: u64,
    pub name: String,
    pub capability: String,
    pub dependencies: Vec<u64>,
}

impl TaskNode {
    pub fn new(
        id: u64,
        name: &str,
        capability: &str,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            capability: capability.to_string(),
            dependencies: Vec::new(),
        }
    }

    pub fn depends_on(
        mut self,
        task_id: u64,
    ) -> Self {
        self.dependencies.push(task_id);
        self
    }
}

pub struct TaskGraph {
    nodes: Vec<TaskNode>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        node: TaskNode,
    ) {
        self.nodes.push(node);
    }

    pub fn nodes(&self) -> &[TaskNode] {
        &self.nodes
    }

    pub fn list(&self) {
        println!();
        println!("======= TASK GRAPH =======");

        for node in &self.nodes {
            println!(
                "#{} {} [{}]",
                node.id,
                node.name,
                node.capability
            );

            if !node.dependencies.is_empty() {
                print!("  Depends on: ");

                for dep in &node.dependencies {
                    print!("{} ", dep);
                }

                println!();
            }
        }

        println!("==========================");
    }
}
