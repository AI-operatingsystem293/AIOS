use std::fs;


pub struct AgentGenerator;


impl AgentGenerator {

    pub fn create(
        name: &str,
    ) -> std::io::Result<()> {


        let root =
            format!(
                "plugins/{}",
                name
            );


        fs::create_dir_all(
            format!(
                "{}/src",
                root
            )
        )?;


        let manifest =
format!(r#"
name = "{}"
version = "0.1.0"
author = "Developer"
description = "AIOS Agent Plugin"

capabilities = [
    "{}"
]
"#,
            name,
            name
        );


        fs::write(
            format!(
                "{}/manifest.toml",
                root
            ),
            manifest
        )?;



        let cargo =
format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"


[lib]
crate-type = ["cdylib"]


[dependencies]
aios = {{
    path = "../../"
}}
"#,
            name
        );


        fs::write(
            format!(
                "{}/Cargo.toml",
                root
            ),
            cargo
        )?;



        let code =
r#"
use aios::sdk::agent::Agent;


pub struct MyAgent;


impl Agent for MyAgent {

    fn name(&self) -> &str {
        "My Agent"
    }


    fn execute(
        &mut self,
        input: String,
    ) -> String {

        format!(
            "Agent received: {}",
            input
        )

    }

}
"#;


        fs::write(
            format!(
                "{}/src/lib.rs",
                root
            ),
            code
        )?;


        println!(
            "✓ Created agent plugin: {}",
            name
        );


        Ok(())

    }

}
