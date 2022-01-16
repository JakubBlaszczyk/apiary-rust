use crate::apiary::Apiary;
use async_graphql::{Context, FieldResult, Object, ID};
use sqlx::PgPool;
pub struct Mutation;

#[Object(extends)]
impl Mutation {
  async fn create_apiary(
    &self,
    ctx: &Context<'_>,
    localization: Option<String>,
    information: Option<String>,
  ) -> FieldResult<Apiary> {
    let pool = ctx.data::<PgPool>().unwrap();

    let row = Apiary::create(&pool, localization.as_ref(), information.as_ref()).await?;
    Ok(row)
  }

  async fn delete_apiary(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
    let pool = ctx.data::<PgPool>().unwrap();
    let id = id.parse::<String>()?;

    Apiary::delete(&pool, &id).await?;
    Ok(true)
  }

  async fn update_apiary(
    &self,
    ctx: &Context<'_>,
    id: ID,
    localization: Option<String>,
    information: Option<String>,
  ) -> FieldResult<Apiary> {
    let pool = ctx.data::<PgPool>().unwrap();
    let id = id.parse::<String>()?;
    let row = Apiary::update(&pool, &id, localization.as_ref(), information.as_ref()).await?;
    Ok(row)
  }
}
