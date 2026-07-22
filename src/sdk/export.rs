use crate::sdk::agent::Agent;

#[macro_export]
macro_rules! export_agent_plugin {

    ($agent:ty) => {

        #[no_mangle]
        pub extern "C" fn create_plugin()
            -> *mut dyn Agent
        {
            let agent =
                <$agent>::new();

            Box::into_raw(
                Box::new(agent)
            )
        }

    };

}
