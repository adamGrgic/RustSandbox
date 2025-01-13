use hyper::{
    Body, Client, Request, Response, Server, Uri,
    client::HttpConnector,
    service::{make_service_fn, service_fn},
};
use hyper::header::CONTENT_TYPE;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    // Proxy server will run on port 43127
    let proxy_port = 43127;

    // The actual backend server (modify as necessary)
    let backend_url = "https://localhost:44375"; // Redirect to actual backend if not intercepted

    // Create a shared Hyper client
    let client = Client::new();

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        let backend_url = backend_url.to_string();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, client.clone(), backend_url.clone())
            }))
        }
    });

    // Start the proxy server
    let addr = ([127, 0, 0, 1], proxy_port).into();
    println!("Proxy server listening on http://localhost:{}", proxy_port);
    if let Err(e) = Server::bind(&addr).serve(make_svc).await {
        eprintln!("Error running proxy server: {}", e);
    }
}

async fn handle_request(
    mut req: Request<Body>,
    client: Client<HttpConnector>,
    backend_url: String,
) -> Result<Response<Body>, hyper::Error> {
    // Check if the request should be intercepted
    if should_intercept(req.uri()) {
        println!("Intercepted request: {}", req.uri());
        return Ok(Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"intercepted": true, "message": "This is dummy data"}"#))
            .unwrap());
    }

    // Forward the request to the actual backend server
    let backend_uri = format!(
        "{}{}",
        backend_url,
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
    );

    println!("Forwarding request to backend: {}", backend_uri);
    *req.uri_mut() = backend_uri.parse::<Uri>().unwrap();

    client.request(req).await
}

// Function to determine if a request should be intercepted
fn should_intercept(uri: &Uri) -> bool {
    let intercept_paths = vec![
        "/ping",  // Add other paths to intercept
    ];

    intercept_paths.iter().any(|p| uri.path().starts_with(p))
}

