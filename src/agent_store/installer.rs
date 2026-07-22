use std::fs;
use std::path::Path;


pub struct AgentInstaller;


impl AgentInstaller {


    pub fn new() -> Self {

        Self

    }



    pub fn install(
        &self,
        source: &str,
    ) -> Result<String, String> {


        let path =
            Path::new(source);


        if !path.exists() {

            return Err(
                "Agent file not found".to_string()
            );

        }



        let filename =
            path.file_name()
            .ok_or(
                "Invalid filename"
            )?
            .to_str()
            .unwrap();



        fs::create_dir_all(
            "plugins"
        )
        .map_err(
            |e| e.to_string()
        )?;



        let destination =
            format!(
                "plugins/{}",
                filename
            );



        fs::copy(
            source,
            &destination,
        )
        .map_err(
            |e| e.to_string()
        )?;



        Ok(
            format!(
                "Installed agent: {}",
                destination
            )
        )

    }

}
