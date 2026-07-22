use crate::{
    kernel::kernel::IKernel,
    master::request::MasterRequest,
};


pub struct Dispatcher<'a> {

    kernel: &'a mut IKernel,

}


impl<'a> Dispatcher<'a> {


    pub fn new(
        kernel: &'a mut IKernel,
    ) -> Self {

        Self {
            kernel,
        }

    }



    pub fn dispatch(
        &mut self,
        input: &str,
    ) -> bool {


        let input = input.trim();


        if input.is_empty() {

            return true;

        }



        let mut parts =
            input.splitn(2, ' ');



        let command =
            parts.next()
            .unwrap_or("");



        let args =
            parts.next()
            .unwrap_or("");




        match command {



            "help" => {

                Self::help();

                true

            }



            "version" => {

                println!(
                    "AIOS v0.0.2 Foundation"
                );

                true

            }



            "agents" => {

                self.kernel
                    .registry()
                    .lock()
                    .unwrap()
                    .list_agents();

                true

            }



            "install" => {

                println!(
                   "Plugin installation is handled through the plugins/ directory."
                );

                println!(
                   "Copy your .so agent into plugins/ and restart AIOS."
                );

                true

            }


            "services" => {

                self.kernel
                    .services()
                    .list();


                true

            }



            "memory" => {


                self.memory_command(args);


                true

            }



            "new-agent" => {


                let parts:
                    Vec<&str> =
                    args.split_whitespace()
                    .collect();



                if parts.is_empty() {


                    println!(
                        "Usage: new-agent <name>"
                    );


                    return true;

                }



                match crate::devtools::generator::AgentGenerator::create(
                    parts[0]
                ) {


                    Ok(_) => {

                        println!(
                            "✓ Agent template created"
                        );

                        true

                    }



                    Err(e) => {

                        println!(
                            "Generator error: {}",
                            e
                        );

                        false

                    }

                }


            }



            "exit" => {


                println!(
                    "Shutting down AIOS..."
                );


                false

            }



            _ => {


                let request =
                    MasterRequest::new(
                        input
                    );



                let response =
                    self.kernel
                    .execute_master(
                        request,
                    );



                println!();


                println!(
                    "{}",
                    response.output
                );


                true

            }

        }

    }





    fn memory_command(
        &self,
        args: &str,
    ) {


        let parts:
            Vec<&str> =
            args.split_whitespace()
            .collect();



        if parts.is_empty() {


            println!(
                "memory list"
            );


            println!(
                "memory get <key>"
            );


            return;

        }




        match parts[0] {


            "list" => {


                self.kernel
                    .memory()
                    .list();

            }



            "get" => {


                if parts.len() != 2 {


                    println!(
                        "Usage: memory get <key>"
                    );


                    return;

                }



                match self.kernel.memory().get(parts[1]) {


                    Some(v) => println!(
                        "{}",
                        v
                    ),



                    None => println!(
                        "Not found"
                    ),

                }


            }



            _ => {


                println!(
                    "Unknown memory command"
                );

            }


        }

    }





    fn help() {


        println!();

        println!(
            "========== AIOS HELP =========="
        );


        println!("help");

        println!("version");

        println!("agents");

        println!("install <agent.so>");

        println!("services");

        println!("memory list");

        println!("memory get <key>");

        println!("new-agent <name>");

        println!("echo <text>");

        println!("add <a> <b>");

        println!("exit");


        println!(
            "==============================="
        );


        println!();

    }

}
