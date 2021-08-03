use anyhow::Context;
use lambda_runtime::handler_fn;
use serde_json::Value;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(test)).await?;
    Ok(())
}

async fn test(_event: Value ,_context: lambda_runtime::Context,) -> Result<(), Error> {
    println!("Hello");
    Ok(())
}