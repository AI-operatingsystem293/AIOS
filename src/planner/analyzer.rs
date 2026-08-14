use crate::planner::goal::Goal;

pub struct GoalAnalyzer;

impl GoalAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, text: &str) -> Goal {
        Goal::new(1, text)
    }

    pub fn intent(&self, text: &str) -> String {
        let input = text.to_lowercase();

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
