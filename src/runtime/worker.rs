use crate::kernel::registry::AgentRegistry;
use std::{
    sync::{
        mpsc::{
            Receiver,
            Sender,
        },
        Arc,
        Mutex,
    },
    thread,
};


use crate::runtime::message::{
    RuntimeMessage,
    ExecutionResult,
};


pub struct Worker {


    id: usize,


}



impl Worker {


    pub fn new(
        id: usize,
    ) -> Self {

        Self {
            id,
        }

    }



    pub fn start(
        self,
        receiver: Arc<Mutex<Receiver<RuntimeMessage>>>,
        result_sender: Sender<ExecutionResult>,
        registry: Arc<Mutex<AgentRegistry>>,

    ) {


        thread::spawn(move || {


            loop {


                let message = {

                    let rx =
                        receiver.lock().unwrap();


                    rx.recv()

                };



                match message {


                    Ok(RuntimeMessage::Execute(job)) => {


                        println!(
                            "[Worker {}] Executing Task {}",
                            self.id,
                            job.task_id
                        );



                        // Temporary execution point.
                        // Registry connection comes next.


                        let output = {

                        let mut registry =
                            registry.lock().unwrap();

                        registry.execute_command(
                            &job.command,
                            &job.input,
                            )

                        };



                        let result =
                            ExecutionResult {

                        task_id:
                            job.task_id,

                        success:
                            output.is_some(),

                        output:
                            output.unwrap_or(
                            "Execution failed".to_string()
                            ),

                         };



                        let _ =
                            result_sender.send(
                                result
                            );


                    }



                    Ok(RuntimeMessage::Stop) => {


                        println!(
                            "Worker {} stopped",
                            self.id
                        );


                        break;


                    }



                    Err(_) => break,


                }

            }


        });


    }


}
