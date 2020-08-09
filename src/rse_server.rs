#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{ Server };
use crate::admin::request;
use std::convert::Infallible;
#[tokio::main]
pub async fn start() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn( |_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async {
            Ok::<_, Infallible>(service_fn(|_req| async {
                request::handler(_req).await
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 9000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
