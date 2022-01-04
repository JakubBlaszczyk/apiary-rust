use crate::apiary::Apiary;
use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

pub struct Query;

#[Object(extends)]
impl Query {
    async fn apiaries<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Apiary>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let rows = Apiary::read_all(&pool).await?;
        Ok(rows)
    }

    async fn apiary<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Apiary> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Apiary::read_one(&pool, &id).await?;
        Ok(row)
    }

    #[graphql(entity)]
    async fn find_apiary_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Apiary> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Apiary::read_one(&pool, &id).await?;
        Ok(row)
    }
}
