use crate::service::service::Service;

pub struct ServiceManager {
    services: Vec<Box<dyn Service>>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register(
        &mut self,
        service: Box<dyn Service>,
    ) {
        self.services.push(service);
    }

    pub fn start_all(&mut self) {
        println!();
        println!("Starting Services");
        println!("-----------------");

        for service in self.services.iter_mut() {
            service.start();

            println!(
                "✓ {} [{}]",
                service.name(),
                service.status(),
            );
        }

        println!();
    }

    pub fn stop_all(&mut self) {
        for service in self.services.iter_mut() {
            service.stop();
        }
    }

    pub fn list(&self) {
        println!();
        println!("Installed Services");
        println!("------------------");

        for service in &self.services {
            println!(
                "{} [{}]",
                service.name(),
                service.status(),
            );
        }

        println!();
    }
}
