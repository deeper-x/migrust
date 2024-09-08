use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Row, Statement};

use crate::{db::models::Migration, db::models::Ping, settings::errors::MyError};

// retrieve ping records list
pub async fn get_ping_records(client: &Client) -> Result<Vec<Ping>, MyError> {
    let _stmt: &str = include_str!("./sql/ping/get_records.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let results: Vec<Ping> = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Ping::from_row_ref(row).unwrap())
        .collect::<Vec<Ping>>();

    Ok(results)
}

// add ping record
pub async fn add_ping_record(client: &Client, ping_info: Ping) -> Result<i64, MyError> {
    let _stmt: &str = include_str!("./sql/ping/add_record.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let rows: Vec<Row> = client.query(&stmt, &[&ping_info.value]).await.unwrap();

    let idx: i64 = rows[0].get(0);

    Ok(idx)
}

pub async fn add_migration_record(
    client: &Client,
    migration_info: Migration,
) -> Result<i64, MyError> {
    let _stmt: &str = include_str!("./sql/migration/add_record.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let rows: Vec<Row> = client
        .query(
            &stmt,
            &[&migration_info.query, &migration_info.project_hash],
        )
        .await
        .unwrap();

    let idx: i64 = rows[0].get(0);

    Ok(idx)
}

pub async fn update_migration_record(
    client: &Client,
    migration_info: Migration,
) -> Result<i64, MyError> {
    let _stmt = include_str!("./sql/migration/update_record.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let rows = client
        .query(
            &stmt,
            &[
                &migration_info.query,
                &migration_info.id,
                &migration_info.project_hash,
            ],
        )
        .await
        .unwrap();

    let idx = rows[0].get(0);

    Ok(idx)
}

pub async fn get_migration_records(
    client: &Client,
    project_id: String,
) -> Result<Vec<Migration>, MyError> {
    let _stmt: &str = include_str!("./sql/migration/get_records.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let results: Vec<Migration> = client
        .query(&stmt, &[&project_id])
        .await?
        .iter()
        .map(|row| Migration::from_row_ref(row).unwrap())
        .collect::<Vec<Migration>>();

    Ok(results)
}
