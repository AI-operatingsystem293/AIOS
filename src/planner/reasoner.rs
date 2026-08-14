use crate::{capability::provider::Provider, planner::goal::Goal};

#[derive(Clone, Debug)]
pub struct PlanningDecision {
    pub capability: String,
    pub input: String,
    pub priority: u8,
}

pub struct Reasoner;

impl Reasoner {
    pub fn new() -> Self {
        Self
    }

    pub fn reason(&self, goal: &Goal, providers: &[Provider]) -> Vec<PlanningDecision> {
        let request = goal.description.to_lowercase();

        let mut scored: Vec<(u32, &Provider)> = providers
            .iter()
            .filter(|provider| provider.healthy)
            .filter_map(|provider| {
                let score = Self::score_provider(provider, &request);

                if score == 0 {
                    None
                } else {
                    Some((score, provider))
                }
            })
            .collect();

        scored.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| a.1.running_tasks.cmp(&b.1.running_tasks))
                .then_with(|| a.1.average_latency_ms.cmp(&b.1.average_latency_ms))
        });

        /*
         * The planner chooses the strongest capability rather than
         * turning every capability exposed by an agent into a task.
         *
         * This preserves multi-agent planning: if several genuinely
         * different providers have strong evidence for the request,
         * they can still participate.
         */
        let mut decisions = Vec::new();

        if let Some((best_score, provider)) = scored.first() {
            let priority = if *best_score >= 100 {
                5
            } else if *best_score >= 60 {
                4
            } else {
                3
            };

            decisions.push(PlanningDecision {
                capability: provider.capability.clone(),
                input: goal.description.clone(),
                priority,
            });

            /*
             * Only add another provider when it has strong independent
             * evidence and is meaningfully close to the best provider.
             */
            for (score, candidate) in scored.iter().skip(1) {
                if *score < 60 {
                    continue;
                }

                if *score * 100 < *best_score * 80 {
                    continue;
                }

                if candidate.capability == provider.capability {
                    continue;
                }

                decisions.push(PlanningDecision {
                    capability: candidate.capability.clone(),
                    input: goal.description.clone(),
                    priority: if *score >= 100 { 5 } else { 4 },
                });
            }
        }

        if decisions.is_empty() {
            decisions.push(PlanningDecision {
                capability: "general_reasoning".to_string(),
                input: goal.description.clone(),
                priority: 1,
            });
        }

        decisions
    }

    fn score_provider(provider: &Provider, request: &str) -> u32 {
        let capability = normalize(&provider.capability);
        let description = normalize(&provider.description);

        let request_tokens = tokenize(request);

        let mut score = 0u32;

        /*
         * Exact capability-name evidence.
         */
        for token in &request_tokens {
            if capability
                .split_whitespace()
                .any(|word| related(word, token))
            {
                score += 100;
            }
        }

        /*
         * Description evidence.
         */
        for token in &request_tokens {
            if description
                .split_whitespace()
                .any(|word| related(word, token))
            {
                score += 15;
            }
        }

        /*
         * Manifest/provider semantic keywords.
         */
        for keyword in &provider.keywords {
            let keyword = normalize(keyword);

            for token in &request_tokens {
                if keyword.split_whitespace().any(|word| related(word, token)) {
                    score += 35;
                }
            }
        }

        /*
         * Generic mathematical-expression evidence.
         *
         * This is based on the request structure rather than matching
         * particular user questions.
         */
        if is_arithmetic_expression(request) {
            match capability.as_str() {
                "add" if contains_addition(request) => score += 120,
                "subtract" if contains_subtraction(request) => score += 120,
                "multiply" if contains_multiplication(request) => score += 120,
                "divide" if contains_division(request) => score += 120,
                "percentage" if contains_percentage(request) => score += 120,
                "power" if contains_power(request) => score += 120,
                "math" => score += 40,
                _ => {}
            }
        }

        /*
         * Provider history.
         */
        score += provider.success_count.min(20) as u32;

        score = score.saturating_sub(provider.failure_count.min(20) as u32 * 5);

        score = score.saturating_sub(provider.running_tasks * 2);

        score
    }
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect()
}

fn tokenize(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .filter(|word| word.len() > 2)
        .map(str::to_string)
        .collect()
}

fn related(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }

    if a.len() >= 4 && b.len() >= 4 {
        if a.starts_with(b) || b.starts_with(a) {
            return true;
        }
    }

    false
}

fn is_arithmetic_expression(request: &str) -> bool {
    let has_number = request.chars().any(|c| c.is_ascii_digit());

    has_number
        && (contains_addition(request)
            || contains_subtraction(request)
            || contains_multiplication(request)
            || contains_division(request)
            || contains_percentage(request)
            || contains_power(request))
}

fn contains_addition(request: &str) -> bool {
    request.contains('+')
        || request.contains(" add ")
        || request.contains(" plus ")
        || request.contains(" added ")
}

fn contains_subtraction(request: &str) -> bool {
    request.contains('-')
        || request.contains(" subtract ")
        || request.contains(" minus ")
        || request.contains(" less ")
}

fn contains_multiplication(request: &str) -> bool {
    request.contains('*')
        || request.contains('×')
        || request.contains(" multiply ")
        || request.contains(" multiplied ")
        || request.contains(" times ")
}

fn contains_division(request: &str) -> bool {
    request.contains('/')
        || request.contains('÷')
        || request.contains(" divide ")
        || request.contains(" divided ")
        || request.contains(" over ")
}

fn contains_percentage(request: &str) -> bool {
    request.contains('%') || request.contains(" percent ") || request.contains(" percentage ")
}

fn contains_power(request: &str) -> bool {
    request.contains('^')
        || request.contains(" power ")
        || request.contains(" exponent ")
        || request.contains(" raised ")
}
