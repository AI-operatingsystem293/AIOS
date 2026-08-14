use crate::sdk::{
    agent::Agent,
    manifest::AgentManifest,
    request::AgentRequest,
    response::AgentResponse,
};

pub struct MathAgent;

impl MathAgent {
    pub fn new() -> Self {
        Self
    }

    fn calculate(&self, input: &str) -> Result<String, String> {
        let text = input.trim();

        if text.is_empty() {
            return Err("No mathematical expression provided.".to_string());
        }

        /*
         * Explicit command:
         *
         * add 2 3
         */
        let mut parts = text.split_whitespace();

        if let Some(command) = parts.next() {
            match command.to_lowercase().as_str() {
                "add" | "sum" => {
                    let a = parse_number(parts.next())?;
                    let b = parse_number(parts.next())?;

                    if parts.next().is_some() {
                        return Err("Usage: add <a> <b>".to_string());
                    }

                    return Ok(format_number(a + b));
                }

                "subtract" | "sub" => {
                    let a = parse_number(parts.next())?;
                    let b = parse_number(parts.next())?;

                    if parts.next().is_some() {
                        return Err("Usage: subtract <a> <b>".to_string());
                    }

                    return Ok(format_number(a - b));
                }

                "multiply" | "mul" => {
                    let a = parse_number(parts.next())?;
                    let b = parse_number(parts.next())?;

                    if parts.next().is_some() {
                        return Err("Usage: multiply <a> <b>".to_string());
                    }

                    return Ok(format_number(a * b));
                }

                "divide" | "div" => {
                    let a = parse_number(parts.next())?;
                    let b = parse_number(parts.next())?;

                    if b == 0.0 {
                        return Err("Division by zero.".to_string());
                    }

                    if parts.next().is_some() {
                        return Err("Usage: divide <a> <b>".to_string());
                    }

                    return Ok(format_number(a / b));
                }

                "percent" | "percentage" => {
                    let percent = parse_number(parts.next())?;
                    let value = parse_number(parts.next())?;

                    if parts.next().is_some() {
                        return Err(
                            "Usage: percent <percentage> <value>".to_string()
                        );
                    }

                    return Ok(format_number((percent / 100.0) * value));
                }

                "power" | "pow" => {
                    let base = parse_number(parts.next())?;
                    let exponent = parse_number(parts.next())?;

                    if parts.next().is_some() {
                        return Err("Usage: power <base> <exponent>".to_string());
                    }

                    return Ok(format_number(base.powf(exponent)));
                }

                _ => {}
            }
        }

        /*
         * Natural mathematical expressions.
         *
         * Examples:
         *
         * 2 + 3
         * 10 - 4
         * 5 * 6
         * 20 / 4
         * 25% of 800
         *
         * This parser intentionally stays small and deterministic.
         */
        let normalized = text
            .replace('%', " % ")
            .replace('+', " + ")
            .replace('-', " - ")
            .replace('*', " * ")
            .replace('/', " / ");

        let tokens: Vec<&str> = normalized.split_whitespace().collect();

        if tokens.len() == 3 {
            let a = tokens[0]
                .parse::<f64>()
                .map_err(|_| "Invalid number.".to_string())?;

            let b = tokens[2]
                .parse::<f64>()
                .map_err(|_| "Invalid number.".to_string())?;

            let result = match tokens[1] {
                "+" => a + b,
                "-" => a - b,
                "*" | "x" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Division by zero.".to_string());
                    }

                    a / b
                }
                "%" => a % b,
                _ => {
                    return Err(
                        "Unsupported mathematical operator.".to_string()
                    )
                }
            };

            return Ok(format_number(result));
        }

        /*
         * Percentage expressions.
         *
         * 25% of 800
         */
        if tokens.len() == 4
            && tokens[1] == "%"
            && tokens[2].eq_ignore_ascii_case("of")
        {
            let percent = tokens[0]
                .parse::<f64>()
                .map_err(|_| "Invalid percentage.".to_string())?;

            let value = tokens[3]
                .parse::<f64>()
                .map_err(|_| "Invalid value.".to_string())?;

            return Ok(format_number((percent / 100.0) * value));
        }

        Err(format!("I could not evaluate: {}", text))
    }
}

impl Agent for MathAgent {
    fn manifest(&self) -> AgentManifest {
    AgentManifest::new()
        .name("Math Agent")
        .version("2.0.0")
        .author("AIOS")
        .description(
            "Performs arithmetic calculations and solves mathematical expressions, \
             including addition, subtraction, multiplication, division, percentages, \
             powers, and numerical calculations.",
        )
        .keywords([
            "math",
            "mathematics",
            "arithmetic",
            "calculate",
            "calculation",
            "number",
            "numbers",
            "add",
            "addition",
            "plus",
            "subtract",
            "subtraction",
            "minus",
            "multiply",
            "multiplication",
            "multiplied",
            "times",
            "divide",
            "division",
            "divided",
            "percentage",
            "percent",
            "power",
            "exponent",
            "expression",
            "+",
            "-",
            "*",
            "/",
            "%",
            "^",
        ])
        .capability("math")
        .capability("add")
        .capability("subtract")
        .capability("multiply")
        .capability("divide")
        .capability("percentage")
        .capability("power")
        }

    fn execute(&mut self, request: AgentRequest) -> AgentResponse {
        match self.calculate(&request.input) {
            Ok(result) => AgentResponse::success(
                request.task_id,
                &result,
            ),

            Err(error) => AgentResponse::error(
                request.task_id,
                &error,
            ),
        }
    }
}

fn parse_number(value: Option<&str>) -> Result<f64, String> {
    value
        .ok_or_else(|| "Missing number.".to_string())?
        .parse::<f64>()
        .map_err(|_| "Invalid number.".to_string())
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}
