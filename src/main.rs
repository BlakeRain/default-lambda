use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let handler = move |event: LambdaEvent<Value>| async move {
        log::info!("Default lambda function handler");
        log::info!("{:#?}", event);
        Result::<(), Error>::Ok(())
    };

    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}
