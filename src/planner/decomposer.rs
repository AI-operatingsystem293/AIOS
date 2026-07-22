use crate::{
    planner::goal::Goal,
    task::task::Task,
};


pub struct TaskDecomposer;


impl TaskDecomposer {


    pub fn new() -> Self {
        Self
    }



    pub fn decompose(
        &self,
        goal: &Goal,
    ) -> Vec<Task> {


        let separators = [
            " and ",
            " then ",
            ",",
            " also ",
        ];



        let mut segments =
            vec![goal.description.as_str()];



        for separator in separators {

            let mut next = Vec::new();


            for segment in segments {

                for part in segment.split(separator) {

                    if !part.trim().is_empty() {

                        next.push(part.trim());

                    }
                }
            }


            segments = next;

        }



        let mut tasks =
            Vec::new();



        let mut id = 1;



        for segment in segments {


            let mut words =
                segment.split_whitespace();



            let command =
                words.next()
                .unwrap_or("")
                .to_string();



            let input =
                words.collect::<Vec<_>>()
                .join(" ");



            tasks.push(
                Task::new(
                    id,
                    None,
                    command,
                    input,
                    1,
                )
            );


            id += 1;

        }



        tasks

    }

}

