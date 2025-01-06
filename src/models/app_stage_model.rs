use crate::models::Item;
use sqlx::{query_as, Error, PgPool};

#[derive(Clone)]
pub struct AppStage {
    pub(crate) db_pool: PgPool,
}

impl AppStage {
    pub async fn create_item(&self, name: &str, description: &str) -> Result<Item, Error> {
        let query = r#"
        INSERT INTO items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#;
        let row: (i32, String, String) = query_as(query)
            .bind(name)
            .bind(description)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(Item {
            id: row.0,
            name: row.1,
            description: row.2,
        })
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, Error> {
        let req = r#"SELECT * FROM items"#;
        let rows = query_as::<_, Item>(req).fetch_all(&self.db_pool).await?;
        Ok(rows)
    }

    pub(crate) async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        let query = r#"SELECT * FROM items WHERE id = $1"#;
        let result = query_as::<_, Item>(query)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(result)
    }

    pub(crate) async fn update_item(
        &self,
        id: i32,
        name: &str,
        description: &str,
    ) -> Result<Option<Item>, Error> {
        let query = r#"
        UPDATE items
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description"#;
        let result = query_as::<_, Item>(query)
            .bind(name)
            .bind(description)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(result)
    }

    pub(crate) async fn delete_item(&self, id: i32) -> Result<bool, Error> {
        let req = r#"DELETE FROM items WHERE id = $1"#;
        let rows = sqlx::query(req).bind(id).execute(&self.db_pool).await?;
        Ok(rows.rows_affected() > 0)
    }

    pub(crate) async fn delete_all_items(&self) -> Result<u64, Error> {
        let req = r#"DELETE FROM items"#;
        let rows = sqlx::query(req).execute(&self.db_pool).await?;
        Ok(rows.rows_affected())
    }
}
