pub struct Validator;


impl Validator {


    pub fn validate(
        path: &str,
    ) -> bool {


        println!(
            "Validating plugin: {}",
            path
        );


        let manifest =
            format!(
                "{}/manifest.toml",
                path
            );


        if std::path::Path::new(
            &manifest
        ).exists()
        {

            println!(
                "✓ Manifest found"
            );

            true

        } else {

            println!(
                "✗ Missing manifest.toml"
            );

            false

        }

    }


    pub fn validate_name(
        name: &str,
    ) -> bool {


        if name.is_empty() {

            println!(
                "Plugin name cannot be empty"
            );

            return false;

        }


        if name.contains(
            ' '
        ) {

            println!(
                "Plugin name cannot contain spaces"
            );

            return false;

        }


        true

    }

}
