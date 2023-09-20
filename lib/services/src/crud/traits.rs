use async_trait::async_trait;
use struct_iterable::Iterable;

use crate::db::DBConnection;
use crate::error::AppError;
use crate::query_param::QueryParams;

#[async_trait]
pub trait Crud<T, F: Iterable, O: Iterable, IdType = i32, L = T> {
    async fn create(&self, db: &DBConnection) -> Result<L, AppError>;

    async fn find_by_id(db: &DBConnection, id: IdType) -> Result<T, AppError>;

    async fn find(db: &DBConnection, params: QueryParams<F, O>) -> Result<Vec<L>, AppError>;

    async fn update(&self, db: &DBConnection, id: IdType) -> Result<(), AppError>;

    async fn delete(db: &DBConnection, id: IdType) -> Result<(), AppError>;
}
