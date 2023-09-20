use actix_web::web::{delete, get, post, put, scope, Data as Extractor, Path, ServiceConfig};
use actix_web::Responder;
use actix_web_validator::{Json, QsQuery};

use configuration::ama::{Ama, FilterColumns, OrderColumns};
use services::db::DBConnection;
use services::error::AppError;
use services::query_param::QueryParams;
use services::response::Response;

pub async fn ama_create_handler(
    db: Extractor<DBConnection>,
    form: Json<Ama>,
) -> Result<impl Responder, AppError> {
    let new_record = form.create(&db).await?;
    Response::result(new_record)
}

pub async fn ama_get_handler(
    db: Extractor<DBConnection>,
    path: Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let result = Ama::find_by_id(&db, id).await?;
    Response::result(result)
}

pub async fn ama_get_all_handler(
    db: Extractor<DBConnection>,
    params: QsQuery<QueryParams<FilterColumns, OrderColumns>>,
) -> Result<impl Responder, AppError> {
    let result = Ama::find(&db, params.into_inner()).await?;
    Response::result(result)
}

pub async fn ama_update_handler(
    db: Extractor<DBConnection>,
    path: Path<i32>,
    form: Json<Ama>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    form.update(&db, id).await?;
    Response::ok()
}

pub async fn ama_delete_handler(
    db: Extractor<DBConnection>,
    path: Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    Ama::delete(&db, id).await?;
    Response::ok()
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ama")
            .route("", post().to(ama_create_handler))
            .route("", get().to(ama_get_all_handler))
            .route("/{id}", get().to(ama_get_handler))
            .route("/{id}", put().to(ama_update_handler))
            .route("/{id}", delete().to(ama_delete_handler)),
    );
}
