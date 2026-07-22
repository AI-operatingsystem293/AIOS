#[derive(Clone, Debug)]
pub struct ExecutionJob {

    pub task_id: u64,

    pub command: String,

    pub input: String,

}


#[derive(Clone, Debug)]
pub struct ExecutionResult {

    pub task_id: u64,

    pub success: bool,

    pub output: String,

}


pub enum RuntimeMessage {

    Execute(ExecutionJob),

    Stop,

}
