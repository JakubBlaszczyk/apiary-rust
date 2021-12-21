use async_graphql::{async_trait, Guard, Context, Result};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Role {
    Admin,
    Beekeeper,
    Worker,
}

struct RoleGuard {
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
