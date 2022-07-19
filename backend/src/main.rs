use std::convert::Infallible;
use std::net::SocketAddr;
use std::env;

use lazy_static::lazy_static;
use http::Uri;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use hyper_tls::HttpsConnector;

type HttpsClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

lazy_static! {
    static ref API_KEY: String = env::var("API_KEY").unwrap();
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(client: HttpsClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut uri_string = req.uri().path_and_query().unwrap().to_string();
    uri_string.push_str(&"&apikey=");
    uri_string.push_str(&API_KEY.to_string());

    let uri = Uri::builder()
        .scheme("https")
        .authority("api.etherscan.io")
        .path_and_query(uri_string)
        .build()
        .unwrap();

    let req = Request::builder()
        .method(req.method())
        .uri(uri)
        .header("Accept", "*/*")
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await;

    resp
}
