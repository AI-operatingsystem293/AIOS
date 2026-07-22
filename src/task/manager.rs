use super::{
    status::TaskStatus,
    task::Task,
};

pub struct TaskManager {
    pub tasks: Vec<Task>,
    next_id: u64,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn completed(&self) -> usize {
        self.tasks
           .iter()
           .filter(|t| matches!(t.status, TaskStatus::Completed))
           .count()
    }

    pub fn failed(&self) -> usize {
        self.tasks
           .iter()
           .filter(|t| matches!(t.status, TaskStatus::Failed))
           .count()
    }

    pub fn create(
        &mut self,
        parent: Option<u64>,
        command: String,
        input: String,
        priority: u8,
    ) -> u64 {

        let id = self.next_id;

        self.next_id += 1;

        self.tasks.push(Task::new(
            id,
            parent,
            command,
            input,
            priority,
        ));

        id
    }

    pub fn assign(
        &mut self,
        id: u64,
        agent: String,
    ) {
        if let Some(task) =
            self.tasks.iter_mut().find(|t| t.id == id)
        {
            task.assigned_agent = Some(agent);
        }
    }

    pub fn enqueue(
        &mut self,
        tasks: Vec<Task>,
    ) {

    for task in tasks {

         self.tasks.push(task);
      }
   }

    pub fn start(&mut self, id: u64) {
        if let Some(task) =
            self.tasks.iter_mut().find(|t| t.id == id)
        {
            task.status = TaskStatus::Running;
        }
    }

    pub fn complete(
        &mut self,
        id: u64,
        result: String,
    ) {
        if let Some(task) =
            self.tasks.iter_mut().find(|t| t.id == id)
        {
            task.status = TaskStatus::Completed;
            task.result = Some(result);
        }
    }

    pub fn list(&self) {

        println!();
        println!("================ TASKS ================");

        for task in &self.tasks {

            println!("Task #{}", task.id);

            println!(" Command  : {}", task.command);

            println!(" Status   : {:?}", task.status);

            println!(" Priority : {}", task.priority);

            println!(
                " Agent    : {}",
                task.assigned_agent
                    .as_deref()
                    .unwrap_or("Unassigned")
            );

            if let Some(result) = &task.result {
                println!(" Result   : {}", result);
            }

            println!("--------------------------------");
        }

        println!();
    }
}
