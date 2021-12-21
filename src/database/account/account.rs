use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/*
CREATE TABLE Account (
  id    UUID NOT NULL DEFAULT gen_random_uuid(),
  login         varchar(255) NOT NULL,
  password    varchar(255) NOT NULL,
  email       varchar(255) NOT NULL,
  privileges varchar(255) NOT NULL,
  CONSTRAINT id
    PRIMARY KEY (id));
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

    pub async fn update(pool: &PgPool, id: &str, password: &str) -> Result<Account> {
        sqlx::query!(
            "UPDATE account SET password=$1 WHERE id = $2",
            password,
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