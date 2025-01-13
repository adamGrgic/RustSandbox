use hyper::{
    client::HttpConnector,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, Uri,
};
use hyper::header:: CONTENT_TYPE;
use hyper::body::to_bytes;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    // Address and port for the proxy server
    let addr = ([172, 22, 35, 15], 8080).into();

    // Shared Hyper HTTP client
    let client = Client::new();

    // Start the server
    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, client.clone())
            }))
        }
    });

    println!("Proxy server running on {}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(
    mut req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>, hyper::Error> {
    // Check if the request matches an endpoint to intercept
    println!("handle_request called");
    if should_intercept(req.uri()) {
        println!("Intercepted request to: {}", req.uri());
        return Ok(Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"dummyKey": "dummyValue"}"#))
            .unwrap());
    }

    // Forward the request to the target ERP system
    let target_base = "http://erp-system.local"; // Replace with actual target base URL
    let target_uri = format!("{}{}", target_base, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));

    println!("Forwarding request to: {}", target_uri);
    *req.uri_mut() = target_uri.parse::<Uri>().unwrap();

    // Send the request to the target server
    let response = client.request(req).await;

    // Handle the response from the target server
    match response {
        Ok(res) => {
            println!("Received response with status: {}", res.status());
            let (parts, body) = res.into_parts();
            let body_bytes = to_bytes(body).await?;
            let response = Response::from_parts(parts, Body::from(body_bytes));
            Ok(response)
        }
        Err(err) => {
            eprintln!("Error forwarding request: {}", err);
            Ok(Response::builder()
                .status(502)
                .body(Body::from("Proxy error"))
                .unwrap())
        }
    }
}

fn should_intercept(uri: &Uri) -> bool {
    // Define paths to intercept
    let intercept_paths = vec![
        "/ping"
    ];
     // Check if the path matches any of the intercept paths
    let path = uri.path(); // `uri.path()` returns a &str
    intercept_paths.iter().any(|p| path.starts_with(p))

}

