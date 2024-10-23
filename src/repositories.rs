use sqlx::PgPool;

use crate::models::*;

pub async fn get_meetups(pool: &PgPool, offset: i64, limit: Option<i64>) -> Result<Vec<Meetup>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new("SELECT * FROM meetups ORDER BY meetup_id");
    query.push(" OFFSET ").push_bind(offset);
    if let Some(limit) = limit { query.push( " LIMIT ").push_bind(limit); }

    let meetups = query.build_query_as::<Meetup>()
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(meetups)
}

pub async fn get_meetup_by_id(pool: &PgPool, id: i64) -> Result<Meetup, sqlx::Error> {
    sqlx::query_as!(
        Meetup,
        r#"SELECT * FROM meetups WHERE meetup_id = $1"#,
        id,
    )
        .fetch_one(pool)
        .await
}

pub async fn create_meetup(pool: &PgPool, title: String, body_text: String) -> Result<i64, sqlx::Error> {
    let id: i64 = sqlx::query_scalar(
        r#"INSERT INTO meetups (title, body_text) VALUES ($1, $2) RETURNING meetup_id"#
    )
        .bind(title)
        .bind(body_text)
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn update_meetup(pool: &PgPool, id: i64, title: String, body_text: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE meetups SET title = $1, body_text = $2 WHERE meetup_id = $3"#,
        title,
        body_text,
        id
    )
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_meetup(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"DELETE FROM meetups WHERE meetup_id = $1"#,
        id
    )
        .execute(pool)
        .await?;

    Ok(())
}