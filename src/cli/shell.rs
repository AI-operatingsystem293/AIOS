use std::io::{self, Write};

use crate::command::dispatcher::Dispatcher;

pub fn start(
    mut dispatcher: Dispatcher,
) {
    loop {
        print!("AIOS> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if !dispatcher.dispatch(&input) {
            break;
        }
    }

    println!("AIOS shutdown complete.");
}
