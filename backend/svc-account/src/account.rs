use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/*
CREATE TABLE ACCOUNT (
  ID    UUID NOT NULL DEFAULT gen_random_uuid(),
  LOGIN         varchar(255) NOT NULL,
  PASSWORD    varchar(255) NOT NULL,
  EMAIL       varchar(255) NOT NULL,
  PRIVILEGES varchar(255) NOT NULL,
  CONSTRAINT ID
    PRIMARY KEY (ID));
*/

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Account {
  pub id: uuid::Uuid,
  login: String,
  password: String,
  email: String,
  privileges: String,
}

impl Account {
  pub async fn create(
    pool: &PgPool,
    login: &str,
    password: &str,
    email: &str,
    privileges: &str,
  ) -> Result<Account> {
    let row = sqlx::query!(
            "INSERT INTO account(Login, password, email, privileges) VALUES ($1, $2, $3, $4) RETURNING id",
            login,
            password,
            email,
            privileges
        )
        .fetch_one(pool)
        .await?;

    Ok(Account {
      id: row.id,
      login: login.to_string(),
      password: password.to_string(),
      email: email.to_string(),
      privileges: privileges.to_string(),
    })
  }

  pub async fn read_one(pool: &PgPool, id: &str) -> Result<Account> {
    let row = sqlx::query_as!(
      Account,
      "SELECT * FROM account WHERE id = $1",
      Uuid::parse_str(id)?
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
  }

  pub async fn read_all(pool: &PgPool) -> Result<Vec<Account>> {
    let rows = sqlx::query_as!(Account, "SELECT * FROM account")
      .fetch_all(pool)
      .await?;

    Ok(rows)
  }

  pub async fn update(
    pool: &PgPool,
    id: &str,
    password: Option<&String>,
    login: Option<&String>,
    email: Option<&String>,
  ) -> Result<Account> {
    let temp = Account::read_one(pool, id).await?;
    let result_password = match password {
      Some(x) => x,
      None => &temp.password,
    };
    let result_email = match email {
      Some(x) => x,
      None => &temp.email,
    };
    let result_login = match login {
      Some(x) => x,
      None => &temp.login,
    };
    sqlx::query!(
      "UPDATE account SET password=$1, login=$2, email=$3  WHERE id = $4",
      result_password,
      result_login,
      result_email,
      Uuid::parse_str(id)?
    )
    .execute(pool)
    .await?;

    Ok(Account::read_one(pool, id).await?)
  }

  pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
    sqlx::query!("DELETE FROM account WHERE id = $1", Uuid::parse_str(id)?)
      .execute(pool)
      .await?;

    Ok(())
  }
}
