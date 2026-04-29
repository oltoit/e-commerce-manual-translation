use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum Role {
    #[serde(rename = "USER")]
    RoleUser = 1,
    #[serde(rename = "ADMIN")]
    RoleAdmin = 2,
}

impl Role {
    pub fn from_str(role: &str) -> Option<Role> {
        match role {
            "USER" => Some(Role::RoleUser),
            "ADMIN" => Some(Role::RoleAdmin),
            _ => None,
        }
    }

    pub fn has_user_permission(&self) -> bool { self >= &Role::RoleUser }
    pub fn has_admin_permission(&self) -> bool { self >= &Role::RoleAdmin }
}