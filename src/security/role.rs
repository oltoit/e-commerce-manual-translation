use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Role {
    #[serde(rename = "USER")]
    RoleUser,
    #[serde(rename = "ADMIN")]
    RoleAdmin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: remove unwrap
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Role {
    pub fn from_str(role: &str) -> Option<Role> {
        match role {
            "USER" => Some(Role::RoleUser),
            "ADMIN" => Some(Role::RoleAdmin),
            _ => None,
        }
    }
}