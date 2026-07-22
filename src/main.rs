pub mod kernel;
pub mod sdk;
pub mod cli;
pub mod agents;
pub mod command;
pub mod master;
pub mod task;
pub mod policy;
pub mod plugin;
pub mod plugins;
pub mod event;
pub mod service;
pub mod capability;
pub mod planner;
pub mod security;
pub mod memory;
pub mod aggregator;
pub mod verification;
pub mod runtime;
pub mod recovery;
pub mod devtools;
	
use command::dispatcher::Dispatcher;
use kernel::kernel::IKernel;

fn main() {
    println!("====================================");
    println!(" AIOS Intelligence Kernel v0.5.0");
    println!(" Codename: MVP");
    println!("====================================");

    let mut kernel = IKernel::new();

    println!("Kernel initialized.");
    println!("Type 'help' for commands.");
    println!();

    let dispatcher = Dispatcher::new(&mut kernel);

    cli::shell::start(dispatcher);

    println!("Goodbye.");
}
