use std::convert::Into;
use std::fmt::Debug;
use std::string::ToString;

use chrono::NaiveDate;
use scooby::postgres::select;
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;
use validator::Validate;

use services::db::DBConnection;
use services::error::AppError;
use services::query_param::{DateFilter, EnumFilter, OrderBy, QueryParams, StringFilter};
use services::Status;

#[derive(Deserialize, Serialize, Debug, Iterable)]
pub struct FilterColumns {
    pub content: StringFilter,
    pub status: EnumFilter<Status>,
    pub effective_date: DateFilter,
}

#[derive(Deserialize, Serialize, Debug, Iterable)]
pub struct OrderColumns {
    pub id: OrderBy,
    pub content: OrderBy,
    pub status: OrderBy,
    pub effective_date: OrderBy,
}

#[derive(Serialize, Deserialize, Validate, sqlx::FromRow, PartialEq, Debug)]
pub struct AmaList {
    pub id: i32,
    pub status: Status,
    pub effective_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Validate, sqlx::FromRow, PartialEq, Debug)]
pub struct Ama {
    pub id: Option<i32>,
    pub name: String,
    pub country: String,
    pub description: String,
}

impl Ama {
    pub async fn create(&self, db: &DBConnection) -> Result<Self, AppError> {
        let result = sqlx::query_as!(
            Self,
            r#"
                WITH new_inserted AS (
                    INSERT INTO ama (name, description, country) VALUES ($1, $2, $3) RETURNING *
                )
                SELECT id, name, description, country FROM new_inserted
            "#,
            self.name,
            self.country,
            self.description
        )
        .fetch_one(db)
        .await?;
        Ok(result)
    }

    pub async fn find_by_id(db: &DBConnection, id: i32) -> Result<Self, AppError> {
        let result = sqlx::query_as!(
            Self,
            r#"SELECT id, name, description, country FROM ama WHERE id = $1"#,
            id
        )
        .fetch_optional(db)
        .await?;
        match result {
            Some(result) => Ok(result),
            None => Err(AppError::NotFound("AMA".into())),
        }
    }

    pub async fn find(
        db: &DBConnection,
        params: QueryParams<FilterColumns, OrderColumns>,
    ) -> Result<Vec<AmaList>, AppError> {
        let query = select("id, status, effective_date").from("ama");
        let (query, args, _) = params.build_query(query, "", 20)?;
        let sql = query.to_string();
        let query = sqlx::query_as_with(&sql, args);
        let result: Vec<AmaList> = query.fetch_all(db).await?;
        Ok(result)
    }

    pub async fn update(&self, db: &DBConnection, id: i32) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            // id, content, content, description, country
            "UPDATE ama SET name = $1, description = $2, country = $3 WHERE id = $4",
            &self.name,
            self.description,
            self.country,
            id
        )
        .execute(db)
        .await?
        .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(AppError::NotFound("AMA".into()))
        }
    }

    pub async fn delete(db: &DBConnection, id: i32) -> Result<(), AppError> {
        let rows_affected = sqlx::query!("DELETE FROM ama WHERE id = $1", id)
            .execute(db)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(AppError::NotFound("AMA".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use services::db::DBConnection;
    use services::query_param::QueryParams;

    use super::Ama;

    #[sqlx::test(migrations = "../../migrations", fixtures("setup-ama"))]
    async fn should_pass_find_all(pool: DBConnection) {
        let params = QueryParams {
            page: Some(1),
            limit: Some(10),
            filter: None,
            filter_type: None,
            meta: None,
            order: None,
        };
        let result = Ama::find(&pool, params).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }
}
