// use lambda_http::aws_lambda_events::event;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
// use anyhow::anyhow;
// use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

///This is a simple multiplier function that return the product
///returns a JSON object with the product of the two numbers
///Takes an event object with two numbers: x, y
#[derive(Deserialize)]
struct Request {
    command: String,
    x: i32,
    y: i32,
}

#[derive(Serialize)]
struct Response {
    function_resp: i32,
}

/// This function takes two integers as input and returns their sum.
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

/// This function takes two integers as input and returns their product.
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful information from the event
    let (event, _content) = event.into_parts();
    let function_resp = match event.command.as_str() {
        "sum" => sum(event.x, event.y),
        "multiply" => multiply(event.x, event.y),
        _ => panic!("Invalid command"),
    };
    // let x = event.payload.x;
    // let y = event.payload.y;s
    
    // // Multiply
    // let product = x * y;
    
    // let resp = Response {
    //     product
    // };
    Ok(Response{function_resp})
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

