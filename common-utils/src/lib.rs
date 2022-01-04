use async_graphql::{async_trait, Context, Guard, Result};
use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Enum, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Beekeeper,
    Worker,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

impl Role {
    pub fn from_string(name: &String) -> Result<Role> {
        match name.as_str() {
            "admin" => Ok(Role::Admin),
            "beekeeper" => Ok(Role::Beekeeper),
            "worker" => Ok(Role::Worker),
            _ => Err("Forbidden".into())
        }
    }
}