use crate::kernel::registry::AgentRegistry;
use crate::runtime::pool::WorkerPool;
use crate::verification::{engine::VerificationEngine, result::VerificationResult};

use crate::{
    aggregator::aggregator::ResultAggregator,
    master::{request::MasterRequest, response::MasterResponse, state::MasterState},
    planner::planner::Planner,
    recovery::{engine::RecoveryEngine, result::RecoveryResult},
    task::{graph::TaskGraph, manager::TaskManager, scheduler::TaskScheduler, status::TaskStatus},
};

pub struct MasterAgent {
    planner: Planner,

    aggregator: ResultAggregator,

    state: MasterState,

    task_graph: TaskGraph,

    task_manager: TaskManager,

    verification: VerificationEngine,

    scheduler: TaskScheduler,

    recovery: RecoveryEngine,
}

impl MasterAgent {
    pub fn new(pool: WorkerPool) -> Self {
    Self {
        planner: Planner::new(),

        aggregator: ResultAggregator::new(),
        state: MasterState::Idle,

        task_graph: TaskGraph::new(),
        task_manager: TaskManager::new(),
        verification: VerificationEngine::new(),

        scheduler: TaskScheduler::new(pool),

        recovery: RecoveryEngine::new(),
    }
}

    pub fn state(&self) -> MasterState {
        self.state
    }

    fn set_state(&mut self, state: MasterState) {
        self.state = state;

        println!("Master State -> {:?}", self.state);
    }

    pub fn execute(
    &mut self,
    request: MasterRequest,
    registry: std::sync::Arc<std::sync::Mutex<AgentRegistry>>,
) -> MasterResponse {
        println!();
        println!("====================================");
        println!("        AIOS MASTER AGENT v4");
        println!("====================================");

        self.aggregator.clear();

        self.task_manager = TaskManager::new();

        self.task_graph = TaskGraph::new();

        // =========================
        // Planning
        // =========================

        self.set_state(MasterState::Planning);

        let workflow = {
    let registry = registry.lock().unwrap();

    self.planner.workflow(&request.input, registry.providers())
};

        println!("Planner created {} workflow task(s).", workflow.tasks.len());

        // =========================
        // Scheduling
        // =========================

        self.set_state(MasterState::Scheduling);

        for plan_task in &workflow.tasks {
            let id = self.task_manager.create(
    None,
    plan_task.capability.clone(),
    request.input.clone(),
    plan_task.priority,
);

            for dependency in &plan_task.dependencies {
                self.task_graph.add_edge(*dependency, id);
            }
        }

        println!("Workflow graph created.");

        // =========================
        // Execution
        // =========================

        self.set_state(MasterState::Executing);

        self.scheduler.tick(&mut self.task_manager, registry.clone());

        // =========================
        // Recovery
        // =========================

        let providers = {
    let registry = registry.lock().unwrap();
    registry.providers().providers().to_vec()
};

let failed_tasks: Vec<(u64, String, String)> = self
    .task_manager
    .tasks
    .iter()
    .filter(|task| matches!(task.status, TaskStatus::Failed))
    .map(|task| {
        (
            task.id,
            task.command.clone(),
            task.assigned_agent
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        )
    })
    .collect();

for (task_id, capability, failed_agent) in failed_tasks {
    self.set_state(MasterState::Recovering);

    println!("Recovering Task #{}...", task_id);

    match self.recovery.recover(
        &providers,
        &capability,
        &failed_agent,
    ) {
                RecoveryResult::Retry => {
                    println!("Recovery: retry requested.");
                }

                RecoveryResult::Recovered => {
                    println!("Recovery succeeded.");

                    if let Some(task) = self
                        .task_manager
                        .tasks
                        .iter_mut()
                        .find(|task| task.id == task_id)
                    {
                        task.status = TaskStatus::Pending;
                    }
                }

                RecoveryResult::Replanned => {
                    println!("Planner requested replanning.");
                }

                RecoveryResult::Failed => {
                    println!("Recovery failed.");
                }
            }
        }

        // =========================
        // Verification
        // =========================

        self.set_state(MasterState::Verifying);

        for task in &self.task_manager.tasks {
            if let Some(result) = &task.result {
                self.aggregator.add(result.clone());
            }
        }

        let output = self.aggregator.merge();

        match self.verification.verify(&output) {
            VerificationResult::Passed => {
                println!("Verification passed.");
            }

            VerificationResult::Failed(reason) => {
                println!("Verification failed.");

                println!("{}", reason);

                return MasterResponse::error(reason);
            }
        }

        println!("Verification completed.");

        self.set_state(MasterState::Completed);

        if output.is_empty() {
            return MasterResponse::ok("Execution completed.");
        }

        MasterResponse::ok(output)
    }

    pub fn recover(&mut self) {
        self.set_state(MasterState::Recovering);

        println!("Recovery Engine started.");
    }

    pub fn fail(&mut self) {
        self.set_state(MasterState::Failed);

        println!("Master execution failed.");
    }

    pub fn reset(&mut self) {
        self.aggregator.clear();

        self.task_manager = TaskManager::new();

        self.task_graph = TaskGraph::new();

        self.set_state(MasterState::Idle);

        println!("Master Agent reset.");
    }
}
