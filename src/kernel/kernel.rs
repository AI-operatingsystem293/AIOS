use crate::runtime::pool::WorkerPool;
use crate::plugins::hello::HelloPlugin;
use crate::event::bus::EventBus;
use crate::event::logger_listener::LoggerListener;

use crate::{
    command::dispatcher::Dispatcher,
    memory::memory::MemoryService,
    master::master::MasterAgent,
    plugin::manager::PluginManager,
    planner::planner::Planner,
    service::{
        logger::LoggerService,
        manager::ServiceManager,
    },
};

use super::{
    logger::Logger,
    neural_bus::NeuralBus,
    registry::AgentRegistry,
    scheduler::Scheduler,
};


pub struct IKernel {

    logger: Logger,

    plugin_manager: PluginManager,

    bus: NeuralBus,

    scheduler: Scheduler,

    event_bus: EventBus,

    planner: Planner,

    registry: std::sync::Arc<std::sync::Mutex<AgentRegistry>>,

    services: ServiceManager,

    memory: MemoryService,

    master: MasterAgent,
}


impl IKernel {


    pub fn new() -> Self {

        let mut services =
            ServiceManager::new();


        services.register(
            Box::new(LoggerService::new())
        );


        let mut event_bus =
            EventBus::new();


        event_bus.subscribe(
            Box::new(LoggerListener)
        );


        let registry =
            std::sync::Arc::new(
                std::sync::Mutex::new(
                    AgentRegistry::new()
                )
            );


        {
            let mut r =
                registry.lock().unwrap();

            r.start();
        }


        let worker_pool =
            WorkerPool::new(
                4,
                registry.clone(),
            );


        let mut plugin_manager =
            PluginManager::new();

        plugin_manager.discover();


        let mut plugin_manager =
            PluginManager::new();

        plugin_manager.discover();


        plugin_manager.load_dynamic_plugins(
            &mut registry.lock().unwrap()
        );


        plugin_manager.register_plugin(
            Box::new(
                HelloPlugin::new()
            )
        );


        plugin_manager.load_plugins(
            &mut registry.lock().unwrap()
        );


        // Load developer plugins

        let mut plugin_manager =
            PluginManager::new();


        plugin_manager.load_plugins(
            &mut registry.lock().unwrap(),
        );


        Self {

            logger: Logger::new(),

            bus: NeuralBus::new(),

            scheduler: Scheduler::new(),

            event_bus,

            planner: Planner::new(),

            registry: registry.clone(),

            services,

            plugin_manager,

            memory: MemoryService::new(),

            master: MasterAgent::new(
                worker_pool
            ),
        }
    }


    pub fn execute_master(
        &mut self,
        request: crate::master::request::MasterRequest,
    ) -> crate::master::response::MasterResponse {

        let mut registry =
            self.registry.lock().unwrap();

        self.master.execute(
            request,
            &mut registry,
        )
    }



    pub fn master_mut(
        &mut self,
    ) -> &mut MasterAgent {

        &mut self.master
    }



    pub fn registry(
        &self,
    ) -> std::sync::Arc<std::sync::Mutex<AgentRegistry>> {

        self.registry.clone()
    }



    pub fn services(
        &self,
    ) -> &ServiceManager {

        &self.services
    }



    pub fn memory(
        &self,
    ) -> &MemoryService {

        &self.memory
    }



    pub fn planner(
        &self,
    ) -> &Planner {

        &self.planner
    }



    pub fn logger(
        &self,
    ) -> &Logger {

        &self.logger
    }



    pub fn bus(
        &self,
    ) -> &NeuralBus {

        &self.bus
    }



    pub fn scheduler(
        &self,
    ) -> &Scheduler {

        &self.scheduler
    }



    pub fn event_bus(
        &self,
    ) -> &EventBus {

        &self.event_bus
    }



    pub fn dispatch(
        &mut self,
        dispatcher: &mut Dispatcher,
        command: &str,
    ) -> Result<(), String> {

        if dispatcher.dispatch(command) {

            Ok(())

        } else {

            Err(
                "Command failed".to_string()
            )
        }
    }
}
