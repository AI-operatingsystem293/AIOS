use crate::security::permission::Permission;

pub struct PermissionManager {
    allowed: Vec<Permission>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            allowed: Vec::new(),
        }
    }

    pub fn allow(&mut self, permission: Permission) {
        if !self.allowed.contains(&permission) {
            self.allowed.push(permission);
        }
    }

    pub fn deny(&mut self, permission: &Permission) {
        self.allowed.retain(|p| p != permission);
    }

    pub fn check(&self, permission: &Permission) -> bool {
        self.allowed.contains(permission)
    }

    pub fn list(&self) {
        println!();
        println!("======= PERMISSIONS =======");

        if self.allowed.is_empty() {
            println!("No permissions granted.");
        } else {
            for permission in &self.allowed {
                println!("{:?}", permission);
            }
        }

        println!("===========================");
    }
}
