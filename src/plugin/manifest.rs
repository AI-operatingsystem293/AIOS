use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct PluginManifest {

    pub name: String,

    pub version: String,

    pub author: String,

    pub description: String,

    pub capabilities: Vec<String>,

}



impl PluginManifest {


    // Backward compatible constructor
    // Used by existing plugins
    pub fn new(
        name: &str,
        version: &str,
        author: &str,
        description: &str,
        capabilities: Vec<&str>,
    ) -> Self {

        Self {

            name: name.to_string(),

            version: version.to_string(),

            author: author.to_string(),

            description: description.to_string(),

            capabilities:
                capabilities
                    .iter()
                    .map(|c| c.to_string())
                    .collect(),

        }

    }



    // Load plugin information from manifest.toml

    pub fn load(
        path: &str,
    ) -> Option<Self> {

        let content =
            std::fs::read_to_string(
                path
            )
            .ok()?;


        toml::from_str(
            &content
        )
        .ok()

    }



    pub fn list(
        &self,
    ) {

        println!();

        println!(
            "========== PLUGIN =========="
        );


        println!(
            "Name: {}",
            self.name
        );


        println!(
            "Version: {}",
            self.version
        );


        println!(
            "Author: {}",
            self.author
        );


        println!(
            "Description: {}",
            self.description
        );


        println!(
            "Capabilities:"
        );


        for capability in &self.capabilities {

            println!(
                " - {}",
                capability
            );

        }


        println!(
            "============================"
        );

    }

}
