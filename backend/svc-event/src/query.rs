use crate::event::Event;
use async_graphql::{Context, FieldResult, Object};
use sqlx::PgPool;

pub struct Query;

#[Object(extends)]
impl Query {
    async fn events<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Event>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let rows = Event::read_all(&pool).await?;
        Ok(rows)
    }

    async fn event<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Event> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Event::read_one(&pool, &id).await?;
        Ok(row)
    }

    #[graphql(entity)]
    async fn find_event_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<Event> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = Event::read_one(&pool, &id).await?;
        Ok(row)
    }
}
