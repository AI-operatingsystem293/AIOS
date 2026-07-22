use crate::plugin::manifest::PluginManifest;
use std::fs;


pub struct PluginDiscovery;


impl PluginDiscovery {


    pub fn new() -> Self {
        Self
    }


    pub fn scan(
        &self,
        path: &str,
    ) -> Vec<PluginManifest> {


        let mut manifests =
            Vec::new();


        if let Ok(entries) =
            fs::read_dir(path)
        {


            for entry in entries.flatten()
            {

                let manifest =
                    entry
                    .path()
                    .join(
                        "manifest.toml"
                    );


                if manifest.exists()
                {

                    if let Some(data) =
                        PluginManifest::load(
                            manifest
                            .to_str()
                            .unwrap()
                        )
                    {

                        manifests.push(
                            data
                        );

                    }

                }

            }

        }


        manifests

    }

}
