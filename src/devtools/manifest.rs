use std::{
    collections::HashMap,
    fs,
};

pub struct Manifest {

    values:
        HashMap<String, String>,
}

impl Manifest {

    pub fn load(
        path: &str,
    ) -> Option<Self> {

        let content =
            fs::read_to_string(path).ok()?;

        let mut values =
            HashMap::new();

        for line in content.lines() {

            let line = line.trim();

            if line.is_empty()
                || line.starts_with('#')
            {
                continue;
            }

            if let Some((k, v)) =
                line.split_once('=')
            {

                values.insert(

                    k.trim().to_string(),

                    v.trim()
                        .trim_matches('"')
                        .to_string(),
                );
            }
        }

        Some(Self { values })
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<&String> {

        self.values.get(key)
    }
}
