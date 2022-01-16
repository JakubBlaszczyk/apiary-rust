use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

use crate::account_apiary::AccountApiary;

pub struct Query;

#[Object(extends)]
impl Query {
  async fn accounts_apiaries<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<AccountApiary>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let rows = AccountApiary::read_all(&pool).await?;
    Ok(rows)
  }

  async fn account_apiary<'a>(
    &self,
    ctx: &'a Context<'_>,
    id_account: Option<String>,
    id_apiary: Option<i32>,
  ) -> FieldResult<Vec<AccountApiary>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let row =
      AccountApiary::read_all_for_ids(&pool, id_account.as_ref(), id_apiary.as_ref()).await?;
    Ok(row)
  }

  #[graphql(entity)]
  async fn find_account_by_id<'a>(
    &self,
    ctx: &'a Context<'_>,
    id_account: String,
    id_apiary: String,
  ) -> FieldResult<AccountApiary> {
    let pool = ctx.data::<PgPool>().unwrap();
    let row = AccountApiary::read_one(&pool, &id_account, &id_apiary).await?;
    Ok(row)
  }
}
