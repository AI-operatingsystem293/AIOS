use crate::devtools::generator::AgentGenerator;


pub struct DevTools;


impl DevTools {


    pub fn execute(
        command: &str,
        args: Vec<&str>,
    ) -> bool {


        match command {


            "new-agent" => {

                if args.is_empty() {

                    println!(
                        "Usage: new-agent <name>"
                    );

                    return false;
                }


                match AgentGenerator::create(
                    args[0]
                ) {

                    Ok(_) => true,


                    Err(error) => {

                        println!(
                            "Generator error: {}",
                            error
                        );

                        false
                    }

                }

            }


            _ => {

                println!(
                    "Unknown developer command"
                );

                false

            }

        }

    }

}
