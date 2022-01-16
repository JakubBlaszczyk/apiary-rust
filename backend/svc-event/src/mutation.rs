use crate::event::Event;
use async_graphql::{Context, FieldResult, Object, ID};
use sqlx::{types::chrono::NaiveDateTime, PgPool};

pub struct Mutation;

#[Object(extends)]
impl Mutation {
  async fn create_event(
    &self,
    ctx: &Context<'_>,
    id_apiary: i32,
    time_start: Option<NaiveDateTime>,
    note: Option<String>,
  ) -> FieldResult<Event> {
    let pool = ctx.data::<PgPool>().unwrap();

    let row = Event::create(&pool, id_apiary, time_start.as_ref(), note.as_ref()).await?;
    Ok(row)
  }

  async fn delete_event(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
    let pool = ctx.data::<PgPool>().unwrap();
    let id = id.parse::<String>()?;

    Event::delete(&pool, &id).await?;
    Ok(true)
  }

  async fn update_event(
    &self,
    ctx: &Context<'_>,
    id: ID,
    id_apiary: Option<i32>,
    time_start: Option<NaiveDateTime>,
    time_end: Option<NaiveDateTime>,
    note: Option<String>,
  ) -> FieldResult<Event> {
    let pool = ctx.data::<PgPool>().unwrap();
    let id = id.parse::<String>()?;
    let row = Event::update(
      &pool,
      &id,
      id_apiary.as_ref(),
      time_start.as_ref(),
      time_end.as_ref(),
      note.as_ref(),
    )
    .await?;
    Ok(row)
  }
}
