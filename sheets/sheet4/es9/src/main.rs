#![allow(dead_code)]

use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Role {
    GUEST,
    USER,
    ADMIN,
}

#[derive(Hash, PartialEq, Eq)]
enum Permission {
    READ,
    WRITE,
    EXECUTE,
}

struct Actions {
    action: String,
    permission: HashMap<Permission, bool>,
}

struct User {
    name: String,
    role: Role,
    actions: Vec<Actions>,
}

trait Auth {
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool;
    fn can_write(&self, string: &str) -> bool;
    fn can_read(&self, string: &str) -> bool;
    fn can_execute(&self, string: &str) -> bool;
}

impl Auth for User {
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool {
        self.actions
            .iter()
            .any(|a| a.action == action && *a.permission.get(permission_type).unwrap_or(&false))
    }

    fn can_write(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::WRITE)
    }

    fn can_read(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::READ)
    }

    fn can_execute(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::EXECUTE)
    }
}

impl Default for Actions {
    fn default() -> Self {
        Actions { 
            action: "".to_string(), 
            permission: HashMap::from([(Permission::READ, false), (Permission::WRITE, false), (Permission::EXECUTE, false)]) 
        }
    }
}

impl Actions {
    fn new(action: String, read: bool, write: bool, execute: bool) -> Self {
        Actions { 
            action, 
            permission: HashMap::from([(Permission::READ, read), (Permission::WRITE, write), (Permission::EXECUTE, execute)]) 
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User { name: "Guest".to_string(), role: Role::GUEST, actions: Vec::new() }
    }
}

impl User {
    fn change_role(&mut self, role: Role)  -> Result<(), String> {
        match self.role {
            Role::ADMIN => { 
                self.role = role; 
                Ok(()) 
            },
            Role::USER | Role::GUEST => { 
                if self.role <= role {
                    self.role = role;
                    return Ok(());
                }
                Err("NOPE".to_string())
            }
        }
    }
}

// assuming you can't have the same action twice else use for_each
fn sudo_change_role(user: &mut User, string: String, permission: Permission, value: bool) {
    user.actions
        .iter_mut()
        .find(|a| a.action == string)
        .and_then(|a| a.permission.insert(permission, value));
}

fn main() {
    println!("{}", Role::GUEST < Role::USER);
}
