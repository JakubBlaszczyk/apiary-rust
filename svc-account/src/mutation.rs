use async_graphql::{Context, FieldResult, Object, ID};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;
use sqlx::PgPool;

use crate::account::Account;

pub struct Mutation;

#[Object(extends)]
impl Mutation {
    async fn create_account(
        &self,
        ctx: &Context<'_>,
        login: String,
        password: String,
        #[graphql(validator(email))] email: String,
        privileges: String,
    ) -> FieldResult<Account> {
        let pool = ctx.data::<PgPool>().unwrap();

        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($pbkdf2-sha256$...)
        let password_hash = Pbkdf2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(&password_hash)?;
        Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .ok();

        let row = Account::create(&pool, &login, &password_hash, &email, &privileges).await?;
        Ok(row)
    }

    async fn delete_account(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        Account::delete(&pool, &id).await?;
        Ok(true)
    }

    async fn update_account(
        &self,
        ctx: &Context<'_>,
        id: ID,
        password: Option<String>,
        email: Option<String>,
        login: Option<String>,
    ) -> FieldResult<Account> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        let result_password: Option<String> = password;
        if let Some(some_password) = &result_password {
            let salt = SaltString::generate(&mut OsRng);

            // Hash password to PHC string ($pbkdf2-sha256$...)
            let password_hash = Pbkdf2
                .hash_password(some_password.as_bytes(), &salt)?
                .to_string();

            // Verify password against PHC string
            let parsed_hash = PasswordHash::new(&password_hash)?;
            Pbkdf2
                .verify_password(some_password.as_bytes(), &parsed_hash)
                .ok();
        }

        let row = Account::update(
            &pool,
            &id,
            result_password.as_ref(),
            login.as_ref(),
            email.as_ref(),
        )
        .await?;
        Ok(row)
    }
}
