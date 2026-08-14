pub mod agents;
pub mod aggregator;
pub mod capability;
pub mod cli;
pub mod command;
pub mod devtools;
pub mod event;
pub mod kernel;
pub mod master;
pub mod memory;
pub mod planner;
pub mod plugin;
pub mod plugins;
pub mod policy;
pub mod recovery;
pub mod runtime;
pub mod sdk;
pub mod security;
pub mod service;
pub mod task;
pub mod verification;

use command::dispatcher::Dispatcher;
use kernel::kernel::IKernel;

fn main() {
    println!("====================================");
    println!(" AIOS Intelligence Kernel v0.6.0");
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
