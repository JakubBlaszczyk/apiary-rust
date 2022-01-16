use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

use crate::account::Account;

pub struct Query;

#[Object(extends)]
impl Query {
  async fn accounts<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Account>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let rows = Account::read_all(&pool).await?;
    Ok(rows)
  }

  async fn account<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Account> {
    let pool = ctx.data::<PgPool>().unwrap();
    let row = Account::read_one(&pool, &id).await?;
    Ok(row)
  }

  #[graphql(entity)]
  async fn find_account_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Account> {
    let pool = ctx.data::<PgPool>().unwrap();
    let row = Account::read_one(&pool, &id).await?;
    Ok(row)
  }
}
