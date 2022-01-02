#[macro_use]
extern crate log;

mod mutation;
mod query;
mod apiary;

use actix_web::web::Data;
use actix_web::{guard, middleware, web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use mutation::Mutation;
use query::Query;

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
}

type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ServiceSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let db_pool = PgPool::connect(&database_url).await?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db_pool)
        .finish();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .route("/ping", web::get().to(ping))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
