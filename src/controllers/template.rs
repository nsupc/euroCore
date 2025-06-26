use crate::core::error::Error;
use crate::types::{request, response};
use sqlx::PgPool;
use sqlx::Row;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub(crate) struct Controller {
    pool: PgPool,
}

impl Controller {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn create(
        &self,
        input: request::Template,
    ) -> Result<response::Template, Error> {
        match sqlx::query(
            "INSERT INTO templates (nation, tgid, key, description) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(input.nation)
        .bind(input.tgid)
        .bind(input.key)
        .bind(input.description)
        .map(map_template)
        .fetch_one(&self.pool)
        .await
        {
            Ok(template) => Ok(template),
            Err(e) => Err(Error::Sql(e)),
        }
    }

    pub(crate) async fn get(&self, id: uuid::Uuid) -> Result<response::Template, Error> {
        match sqlx::query("SELECT * FROM templates WHERE id = $1;")
            .bind(id)
            .map(map_template)
            .fetch_one(&self.pool)
            .await
        {
            Ok(template) => Ok(template),
            Err(sqlx::Error::RowNotFound) => Err(Error::TemplateNotFound),
            Err(e) => Err(Error::Sql(e)),
        }
    }

    pub(crate) async fn update(
        &self,
        id: uuid::Uuid,
        input: request::Template,
    ) -> Result<response::Template, Error> {
        Ok(sqlx::query(
            "UPDATE templates SET nation = $1, tgid = $2, key = $3, description = $4, modified_at = $5 WHERE id = $6 RETURNING *;",
        )
        .bind(input.nation)
        .bind(input.tgid)
        .bind(input.key)
        .bind(input.description)
        .bind(chrono::Utc::now())
        .bind(id)
        .map(map_template)
        .fetch_one(&self.pool)
        .await?)
    }
}

fn map_template(row: PgRow) -> response::Template {
    response::Template {
        id: row.get("id"),
        nation: row.get("nation"),
        tgid: row.get("tgid"),
        key: row.get("key"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        modified_at: row.get("modified_at"),
    }
}
