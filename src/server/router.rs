use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::db::{dml, models};
use crate::settings;

// retrives ping records
pub async fn get_ping_records(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let pings: Vec<models::Ping> = dml::get_ping_records(&client).await?;

    Ok(HttpResponse::Ok().json(pings))
}

// insert ping record
pub async fn add_ping_record(
    ping: web::Json<models::Ping>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let ping_info: models::Ping = ping.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let new_ping: i64 = dml::add_ping_record(&client, ping_info).await?;

    Ok(HttpResponse::Ok().json(new_ping))
}

pub async fn add_migration_record(
    migration: web::Json<models::Migration>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let migration_info: models::Migration = migration.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let new_migration: i64 = dml::add_migration_record(&client, migration_info).await?;

    Ok(HttpResponse::Ok().json(new_migration))
}

pub async fn get_migrations_records(
    db_pool: web::Data<Pool>,
    migr_data: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;

    let in_migr_data = migr_data.into_inner();

    let migrations: Vec<models::Migration> =
        dml::get_migration_records(&client, in_migr_data.0).await?;

    Ok(HttpResponse::Ok().json(migrations))
}
