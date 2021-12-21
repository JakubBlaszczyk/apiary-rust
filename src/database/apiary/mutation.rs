use crate::database::apiary::apiary::Apiary;
use async_graphql::{Context, FieldResult, Object, ID};
use sqlx::PgPool;
pub struct Mutation;

#[Object(extends)]
impl Mutation {
    async fn create_apiary(
        &self,
        ctx: &Context<'_>,
        localization: String,
        information: String,
    ) -> FieldResult<Apiary> {
        let pool = ctx.data::<PgPool>().unwrap();

        let row = Apiary::create(&pool, &localization, &information).await?;
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
        localization: String,
        information: String,
    ) -> FieldResult<Apiary> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;
        let row = Apiary::update(&pool, &id, &localization, &information).await?;
        Ok(row)
    }
}
