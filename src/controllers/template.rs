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
            "INSERT INTO templates (nation, tgid, key) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(input.nation)
        .bind(input.tgid)
        .bind(input.key)
        .map(map_template)
        .fetch_one(&self.pool)
        .await
        {
            Ok(template) => Ok(template),
            Err(e) => Err(Error::Sql(e)),
        }
    }
}

fn map_template(row: PgRow) -> response::Template {
    response::Template {
        id: row.get("id"),
        nation: row.get("nation"),
        tgid: row.get("tgid"),
        key: row.get("key"),
        created_at: row.get("created_at"),
        modified_at: row.get("modified_at"),
    }
}
