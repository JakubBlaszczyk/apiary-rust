use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, FromRow, PgPool};

/*
CREATE TABLE EVENT (
  ID              SERIAL NOT NULL,
  ID_apiary       int4 NOT NULL,
  Time_start      timestamp,
  Time_end        timestamp,
  note            varchar(1023),
  PRIMARY KEY (ID));
*/

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Event {
  pub id: i32,
  id_apiary: i32,
  time_start: Option<NaiveDateTime>,
  time_end: Option<NaiveDateTime>,
  note: Option<String>,
}

impl Event {
  pub async fn create(
    pool: &PgPool,
    id_apiary: i32,
    time_start: Option<&NaiveDateTime>,
    note: Option<&String>,
  ) -> Result<Event> {
    let row = sqlx::query!(
      "INSERT INTO event(id_apiary, time_start, note) VALUES ($1, $2, $3) RETURNING id",
      id_apiary,
      time_start,
      note
    )
    .fetch_one(pool)
    .await?;

    Ok(Event {
      id: row.id,
      id_apiary: id_apiary,
      time_start: time_start.cloned(),
      time_end: None,
      note: note.cloned(),
    })
  }

  pub async fn read_one(pool: &PgPool, id: &str) -> Result<Event> {
    let row = sqlx::query_as!(
      Event,
      "SELECT * FROM event WHERE id = $1",
      id.parse::<i32>().unwrap()
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
  }

  pub async fn read_all(pool: &PgPool) -> Result<Vec<Event>> {
    let rows = sqlx::query_as!(Event, "SELECT * FROM event")
      .fetch_all(pool)
      .await?;

    Ok(rows)
  }

  pub async fn update(
    pool: &PgPool,
    id: &str,
    id_apiary: Option<&i32>,
    time_start: Option<&NaiveDateTime>,
    time_end: Option<&NaiveDateTime>,
    note: Option<&String>,
  ) -> Result<Event> {
    let temp = Event::read_one(pool, id).await?;
    let result_apiary_id = match id_apiary {
      Some(x) => x,
      None => &temp.id_apiary,
    };
    let result_time_start = match time_start {
      Some(x) => Some(x),
      None => temp.time_start.as_ref(),
    };
    let result_time_end = match time_end {
      Some(x) => Some(x),
      None => temp.time_end.as_ref(),
    };
    let result_note = match note {
      Some(x) => Some(x),
      None => temp.note.as_ref(),
    };
    sqlx::query!(
      "UPDATE event SET id_apiary=$1, time_start=$2, time_end=$3, note=$4 WHERE id = $5",
      result_apiary_id,
      result_time_start,
      result_time_end,
      result_note,
      id.parse::<i32>().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(Event::read_one(pool, id).await?)
  }

  pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
    sqlx::query!(
      "DELETE FROM apiary WHERE id = $1",
      id.parse::<i32>().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(())
  }
}
