// better-logger/tests/http_server/main.rs

// In a seperate terminal:
// cargo build --bin http-test --features testing-http
// cargo run --bin http-test --features testing-http
// run the native or wasm tests witn network_logs as true
// logs should print to your http server terminal
// optional second argument: the status to answer POST with (default 204), e.g. 429, 401, 500
// cargo run --bin http-test --features testing-http 8070 429

#[cfg(feature = "testing-http")]
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = std::env::args().nth(1).unwrap_or_else(|| "8090".into());
    let status: u16 = match std::env::args().nth(2) {
        Some(value) => value.parse()?,
        None => 204,
    };
    let addr = format!("127.0.0.1:{port}");

    println!("listening on http://{addr}/ (answering POST with {status})");

    let server = tiny_http::Server::http(&addr)?;
    loop {
        let mut request = server.recv()?;

        if request.method() == &tiny_http::Method::Options {
            request.respond(with_cors(tiny_http::Response::empty(204))?)?;
            continue;
        }

        if request.method() != &tiny_http::Method::Post {
            request.respond(tiny_http::Response::empty(405))?;
            continue;
        }

        let mut body = String::new();
        request.as_reader().read_to_string(&mut body)?;
        println!("{body}");

        request.respond(with_cors(tiny_http::Response::empty(status))?)?;
    }
}

#[cfg(feature = "testing-http")]
fn with_cors(response: tiny_http::Response<std::io::Empty>) -> Result<tiny_http::Response<std::io::Empty>, Box<dyn std::error::Error + Send + Sync>> {
    let mut response = response;
    for (name, value) in [
        ("Access-Control-Allow-Origin", "*"),
        ("Access-Control-Allow-Methods", "POST, OPTIONS"),
        ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
    ] {
        let header = tiny_http::Header::from_bytes(name.as_bytes(), value.as_bytes())
            .map_err(|_| format!("invalid header {name}"))?;
        response = response.with_header(header);
    }
    return Ok(response);
}