use std::env::var;

use actix_web::dev::{Service, ServiceResponse};
use actix_web::web::get;
use actix_web::{web, App, HttpResponse, HttpServer};
use lambda_web::{run_actix_on_lambda, LambdaError};
use serde_json::json;

use services::error::actix_error_handler;

mod ama;
mod authorization;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    services::load_env(Some(vec!["ENC_KEY"]));
    // get env variable
    // create a connection pool to use in all the routes
    let db = services::db::Connection::lazy().expect("Database connection failed.");

    let factory = move || {
        App::new()
            .wrap_fn(|request, srv| {
                let is_ok = services::middleware::Middleware::check_login(&request);
                if is_ok {
                    srv.call(request)
                } else {
                    let response =
                        HttpResponse::Unauthorized().json(json!({"error": "Session expired"}));
                    let (request, _) = request.into_parts();
                    let response = ServiceResponse::new(request, response);
                    Box::pin(async { Ok(response) })
                }
            })
            .app_data(web::Data::new(db.clone()))
            .app_data(actix_error_handler())
            .route("/", get().to(HttpResponse::Ok))
            .configure(ama::routes)
            .configure(authorization::routes)
    };
    if var("LAMBDA_RUNTIME_API").is_ok() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?
    } else {
        println!("app started http://127.0.0.1:8080");
        // Run local server
        HttpServer::new(factory)
            .bind(("0.0.0.0", 8080))?
            .run()
            .await?;
    }
    Ok(())
}
