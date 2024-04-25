use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;



async fn handle_request(req: Request) -> Result<Response<Body>, Error> {
    // set the request from users or the default input
    // let request = req.query_string_parameters_ref().and_then(|params| params.first("text"))   // set the keyword "text" for customized input text 
    //     .unwrap_or("This is a new story");  // set the default input text for generation

    let request = req.query_string_parameters_ref().and_then(|params| params.first("text")).unwrap(); 

    // print related messages for success or error
    let message = match generate_response(request.to_string()) {
        Ok(result) => result,
        Err(e) => format!("Inference error: {:?}", e),
    };
    // print out the generated response for visualzation
    println!("Response from model: {:?}", message);

    // build HTTP Response Body 
    let response = Response::builder().status(200).header("content-type", "text/html").body(Body::from(message)).map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(handle_request)).await
}
