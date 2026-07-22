use std::sync::{
    Arc,
    Mutex,
    mpsc,
};

use crate::{
    kernel::registry::AgentRegistry,

    runtime::{
        message::{
            RuntimeMessage,
            ExecutionJob,
            ExecutionResult,
        },

        worker::Worker,
    },
};



pub struct WorkerPool {

    sender:
        mpsc::Sender<RuntimeMessage>,


    pub results:
        Arc<Mutex<mpsc::Receiver<ExecutionResult>>>,

}



impl WorkerPool {


    pub fn new(
        count: usize,
        registry: Arc<Mutex<AgentRegistry>>,
    ) -> Self {


        let (tx, rx) =
            mpsc::channel();



        let (result_tx, result_rx) =
            mpsc::channel();



        let receiver =
            Arc::new(
                Mutex::new(rx)
            );



        for id in 0..count {


            let worker =
                Worker::new(
                    id + 1
                );


            worker.start(

                receiver.clone(),

                result_tx.clone(),

                registry.clone(),

            );


        }



        println!(
            "✓ Worker Pool started with {} workers",
            count
        );



        Self {

            sender: tx,


            results:
                Arc::new(
                    Mutex::new(
                        result_rx
                    )
                ),

        }


    }



    pub fn submit(

        &self,

        job: ExecutionJob,

    ) {


        let _ =
            self.sender.send(
                RuntimeMessage::Execute(job)
            );


    }


}
