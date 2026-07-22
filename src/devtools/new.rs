use std::fs;


pub struct NewAgent;


impl NewAgent {


    pub fn create(
        name: &str,
    ) -> std::io::Result<()> {


        let path =
            format!(
                "plugins/{}",
                name
            );


        fs::create_dir_all(
            format!(
                "{}/src",
                path
            )
        )?;


        fs::write(
            format!(
                "{}/manifest.toml",
                path
            ),
            format!(
r#"name = "{}"
version = "0.1.0"
author = "Developer"
description = "AIOS Agent Plugin"

capabilities = [
    "{}"
]
"#,
                name,
                name
            )
        )?;


        fs::write(
            format!(
                "{}/Cargo.toml",
                path
            ),
r#"[package]
name = "agent_plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
aios = { path = "../.." }
"#
        )?;


        let struct_name =
            Self::to_struct_name(name);


        fs::write(
            format!(
                "{}/src/lib.rs",
                path
            ),
            format!(
r#"use aios::sdk::agent::Agent;


pub struct {};


impl {{

    pub fn new() -> Self {{
        Self
    }}

}}


impl Agent for {{

    fn name(&self) -> &str {{
        "{}"
    }}

}}
"#,
                struct_name,
                name
            )
        )?;


        println!(
            "✓ Created agent plugin: {}",
            name
        );


        Ok(())

    }



    fn to_struct_name(
        name: &str,
    ) -> String {

        name
            .split('_')
            .map(|part| {

                let mut chars =
                    part.chars();

                match chars.next() {

                    Some(first) => {

                        first.to_uppercase()
                            .collect::<String>()
                            + chars.as_str()

                    }

                    None => String::new(),

                }

            })
            .collect()

    }

}
