use crate::{kernel::kernel::IKernel, master::request::MasterRequest};
use std::env;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub struct Dispatcher<'a> {
    kernel: &'a mut IKernel,
}

impl<'a> Dispatcher<'a> {
    pub fn new(kernel: &'a mut IKernel) -> Self {
        Self { kernel }
    }

    fn update() {
        println!();
        println!("========== AIOS UPDATE ==========");
        println!("Downloading latest AIOS release...");

        let exe = match env::current_exe() {
            Ok(path) => path,
            Err(e) => {
                println!("✗ Cannot locate AIOS executable: {}", e);
                return;
            }
        };

        let tmp = exe.with_extension("update");

        let url = "https://github.com/AI-operatingsystem293/AIOS/releases/download/v0.6.0/aios-linux-aarch64";

        let download = Command::new("curl")
            .args(["-fL", url, "-o"])
            .arg(&tmp)
            .status();

        match download {
            Ok(status) if status.success() => {
                println!("✓ Latest AIOS downloaded");
            }
            Ok(status) => {
                println!("✗ Download failed: {}", status);
                let _ = std::fs::remove_file(&tmp);
                return;
            }
            Err(e) => {
                println!("✗ Failed to run curl: {}", e);
                let _ = std::fs::remove_file(&tmp);
                return;
            }
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Err(e) = std::fs::set_permissions(
                &tmp,
                std::fs::Permissions::from_mode(0o755),
            ) {
                println!("✗ Cannot make updated AIOS executable: {}", e);
                let _ = std::fs::remove_file(&tmp);
                return;
            }
        }

        println!("✓ Installing update...");

        if let Err(e) = std::fs::rename(&tmp, &exe) {
            println!("✗ Cannot install update: {}", e);
            let _ = std::fs::remove_file(&tmp);
            return;
        }

        println!("✓ AIOS updated successfully");
        println!("Starting updated AIOS...");
        println!();

        #[cfg(unix)]
        {
            let err = Command::new(&exe).exec();
            println!("✗ Failed to restart AIOS: {}", err);
        }

        #[cfg(not(unix))]
        {
            match Command::new(&exe).status() {
                Ok(_) => {}
                Err(e) => println!("✗ Failed to restart AIOS: {}", e),
            }
        }
    }

    pub fn dispatch(&mut self, input: &str) -> bool {
        let input = input.trim();

        if input.is_empty() {
            return true;
        }

        let mut parts = input.splitn(2, ' ');

        let command = parts.next().unwrap_or("");

        let args = parts.next().unwrap_or("");

        match command {
            "help" => {
                Self::help();

                true
            }

            "version" => {
                println!("AIOS v0.6.0 Foundation");

                true
            }
 
            "update" => {
    Self::update();
    true
}

            "agents" => {
                self.kernel.registry().lock().unwrap().list_agents();

                true
            }

            "install" => {
                println!("Plugin installation is handled through the plugins/ directory.");

                println!("Copy your .so agent into plugins/ and restart AIOS.");

                true
            }

            "services" => {
                self.kernel.services().list();

                true
            }

            "memory" => {
                self.memory_command(args);

                true
            }

            "new-agent" => {
                let parts: Vec<&str> = args.split_whitespace().collect();

                if parts.is_empty() {
                    println!("Usage: new-agent <name>");

                    return true;
                }

                match crate::devtools::generator::AgentGenerator::create(parts[0]) {
                    Ok(_) => {
                        println!("✓ Agent template created");

                        true
                    }

                    Err(e) => {
                        println!("Generator error: {}", e);

                        false
                    }
                }
            }


            "exit" => {
                println!("Shutting down AIOS...");

                false
            }

            _ => {
                // First try a registered capability/command.
                let registry = self.kernel.registry();

                let result = {
                    let mut registry = registry.lock().unwrap();

                    registry.execute_command(command, args)
                };

                if let Some(output) = result {
                    println!("{}", output);
                    return true;
                }

                // ---------------------------------------------------------
                // NATURAL LANGUAGE MODE
                //
                // Natural-language requests go through the Master.
                // The Planner decides which capabilities are required.
                //
                // ---------------------------------------------------------

                let request = MasterRequest::new(input);
                let response = self.kernel.execute_master(request);

                println!();
                println!("{}", response.output);

                true
            }
        }
    }

    fn memory_command(&self, args: &str) {
        let parts: Vec<&str> = args.split_whitespace().collect();

        if parts.is_empty() {
            println!("memory list");

            println!("memory get <key>");

            return;
        }

        match parts[0] {
            "list" => {
                self.kernel.memory().list();
            }

            "get" => {
                if parts.len() != 2 {
                    println!("Usage: memory get <key>");

                    return;
                }

                match self.kernel.memory().get(parts[1]) {
                    Some(v) => println!("{}", v),

                    None => println!("Not found"),
                }
            }

            _ => {
                println!("Unknown memory command");
            }
        }
    }

    fn help() {
        println!();

        println!("========== AIOS HELP ==========");

        println!("help");

        println!("version");

        println!("update");

        println!("agents");

        println!("install <agent.so>");

        println!("services");

        println!("memory list");

        println!("memory get <key>");

        println!("new-agent <name>");

        println!("echo <text>");

        println!("add <a> <b>");

        println!("exit");

        println!("===============================");

        println!();
    }
}
