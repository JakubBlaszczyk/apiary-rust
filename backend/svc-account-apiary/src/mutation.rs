use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

use crate::account_apiary::AccountApiary;

pub struct Mutation;

#[Object(extends)]
impl Mutation {
  async fn create_account_apiary(
    &self,
    ctx: &Context<'_>,
    id_account: String,
    id_apiary: i32,
  ) -> FieldResult<AccountApiary> {
    let pool = ctx.data::<PgPool>().unwrap();

    let row = AccountApiary::create(&pool, &id_account, &id_apiary).await?;
    Ok(row)
  }

  async fn delete_account_apiary(
    &self,
    ctx: &Context<'_>,
    id_account: String,
    id_apiary: i32,
  ) -> FieldResult<bool> {
    let pool = ctx.data::<PgPool>().unwrap();

    AccountApiary::delete(&pool, &id_account, &id_apiary).await?;
    Ok(true)
  }
}
