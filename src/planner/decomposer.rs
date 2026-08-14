use crate::{planner::goal::Goal, task::task::Task};

pub struct TaskDecomposer;

impl TaskDecomposer {
    pub fn new() -> Self {
        Self
    }

    pub fn decompose(&self, goal: &Goal) -> Vec<Task> {
        let input = goal.description.as_str();
        let intent = self.detect_intent(input);

        let capabilities = match intent.as_str() {
            "website_development" => vec![
                "planning",
                "web_research",
                "code_generation",
                "file_generation",
                "testing",
            ],

            "social_application_development" => vec![
                "planning",
                "web_research",
                "architecture",
                "ui_development",
                "backend_development",
                "database",
                "authentication",
                "code_generation",
                "testing",
            ],

            "software_development" => {
                vec!["planning", "architecture", "code_generation", "testing"]
            }

            "web_research" => vec!["web"],

            "question_answering" => vec!["general_reasoning"],

            _ => vec!["general_reasoning"],
        };

        capabilities
            .into_iter()
            .enumerate()
            .map(|(index, capability)| {
                Task::new(
                    (index + 1) as u64,
                    None,
                    capability.to_string(),
                    input.to_string(),
                    1,
                )
            })
            .collect()
    }

    fn detect_intent(&self, input: &str) -> String {
        let input = input.to_lowercase();

        if contains_any(
            &input,
            &[
                "build a social app",
                "create a social app",
                "make a social network",
                "build social network",
            ],
        ) {
            return "social_application_development".to_string();
        }

        if contains_any(
            &input,
            &[
                "build a website",
                "create a website",
                "make a website",
                "build website",
                "create website",
            ],
        ) {
            return "website_development".to_string();
        }

        if contains_any(
            &input,
            &[
                "write code",
                "write a program",
                "build an app",
                "create an app",
                "develop an application",
            ],
        ) {
            return "software_development".to_string();
        }

        if contains_any(
            &input,
            &["search", "research", "latest", "news", "find information"],
        ) {
            return "web_research".to_string();
        }

        if contains_any(
            &input,
            &[
                "tell me", "what is", "what are", "explain", "how does", "why does",
            ],
        ) {
            return "question_answering".to_string();
        }

        "general".to_string()
    }
}

fn contains_any(input: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| input.contains(pattern))
}
