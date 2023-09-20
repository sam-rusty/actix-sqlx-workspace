use aws_lambda_events::sqs::SqsEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};

use services::db::DBConnection;
use services::queue::Message;

async fn process_message(_db: &DBConnection, body: &str) -> Result<(), Error> {
    let _message = serde_json::from_str::<Message>(body)?;
    Ok(())
}

async fn sqs_event_handler(db: &DBConnection, event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    for record in event.payload.records {
        if let Some(body) = &record.body {
            let result = process_message(db, body).await;
            if let Err(e) = result {
                // todo:: Notify Sentry
                println!("[Error] {e} - {body}");
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // create shared db connection pool
    let db = services::db::Connection::lazy()?;
    let db_pool = &db;
    // run event handler with shared db pool as ref
    lambda_runtime::run(service_fn(move |event: LambdaEvent<SqsEvent>| async move {
        sqs_event_handler(db_pool, event).await
    }))
    .await?;
    Ok(())
}
