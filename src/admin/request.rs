use hyper::{Body, Request, Response, Server, StatusCode, http};
use std::convert::Infallible;
use crate::constants;
use crate::admin::permalink::extract_url;
use crate::admin::template::load_page;

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:?}", req);
    let config_exist: bool = std::path::Path::new(constants::CONFIG_FILE).exists();
    if !config_exist && req.uri() != "/rs-admin/install" {
        return Ok(navigate_url(req, "rs-admin/install"));
    }
    let params = extract_url(req.uri().path().clone(),"PostName");
    println!("{:?}", params);
    let template = load_page(params);
    let builder = Response::builder()
        .status(StatusCode::from_u16(template.0).unwrap());

    //if url != req.uri(){
    //    builder.header("Location", url);
    //}


    Ok(builder.body(Body::from(template.1)).unwrap())
}

pub fn navigate_url(req: Request<Body>, url: &str) -> Response<Body> {
    if url == req.uri() {
        return create_request_builder(200).body(Body::from("")).unwrap();
    }
    let builder = create_request_builder(302).
        header("Location", url);
    return builder.body(Body::from("")).unwrap();
}

pub fn create_request_builder(status_code:u16) -> http::response::Builder {
    let res = Response::builder().status(StatusCode::from_u16(status_code).unwrap());
    return res;
}