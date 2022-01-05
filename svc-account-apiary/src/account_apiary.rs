use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/*
CREATE TABLE ACCOUNT_APIARY (
  ID_account UUID NOT NULL,
  ID_apiary   int4 NOT NULL,
  PRIMARY KEY (ID_account,
  ID_apiary));
ALTER TABLE ACCOUNT_APIARY ADD CONSTRAINT FKAccou_apia926913 FOREIGN KEY (ID_apiary) REFERENCES APIARY (ID);
ALTER TABLE ACCOUNT_APIARY ADD CONSTRAINT FKAccou_apia442043 FOREIGN KEY (ID_account) REFERENCES ACCOUNT (ID);
*/

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct AccountApiary {
    id_account: uuid::Uuid,
    id_apiary: i32,
}

impl AccountApiary {
    pub async fn create(pool: &PgPool, id_account: &str, id_apiary: &i32) -> Result<AccountApiary> {
        sqlx::query!(
            "INSERT INTO account_apiary(id_account, id_apiary) VALUES ($1, $2)",
            Uuid::parse_str(id_account)?,
            id_apiary,
        )
        .fetch_one(pool)
        .await?;

        Ok(AccountApiary {
            id_account: Uuid::parse_str(id_account).unwrap(),
            id_apiary: id_apiary.clone(),
        })
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<AccountApiary>> {
        let rows = sqlx::query_as!(AccountApiary, "SELECT * FROM account_apiary")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn read_all_for_ids(
        pool: &PgPool,
        id_account: Option<&String>,
        id_apiary: Option<&i32>,
    ) -> Result<Vec<AccountApiary>> {
        if let Some(id) = id_account {
            let rows = sqlx::query_as!(
                AccountApiary,
                "SELECT * FROM account_apiary WHERE id_account = $1",
                Uuid::parse_str(id)?
            )
            .fetch_all(pool)
            .await?;
            return Ok(rows);
        }
        if let Some(id) = id_apiary {
            let rows = sqlx::query_as!(
                AccountApiary,
                "SELECT * FROM account_apiary WHERE id_apiary = $1",
                id
            )
            .fetch_all(pool)
            .await?;
            return Ok(rows);
        }

        panic!("Can't retrieve value")
    }

    pub async fn read_one(
        pool: &PgPool,
        id_account: &str,
        id_apiary: &str,
    ) -> Result<AccountApiary> {
        let row = sqlx::query_as!(
            AccountApiary,
            "SELECT * FROM account_apiary WHERE id_account = $1 and id_apiary = $2",
            Uuid::parse_str(id_account)?,
            id_apiary.parse::<i32>().unwrap()
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id_account: &str, id_apiary: &i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM account_apiary WHERE id_account = $1 and id_apiary = $2",
            Uuid::parse_str(id_account)?,
            id_apiary
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
