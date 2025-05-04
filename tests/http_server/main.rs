// better-logger/tests/http_server/main.rs

// In a seperate terminal:
// cargo build --bin http-test --features testing-http
// cargo run --bin http-test --features testing-http
// run the native or wasm tests witn network_logs as true
// logs should print to your http server terminal

#[cfg(feature = "testing-http")]
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = std::env::args().nth(1).unwrap_or_else(|| "8090".into());
    let addr = format!("127.0.0.1:{port}");

    println!("listening on http://{addr}/ ");

    let server = tiny_http::Server::http(&addr)?;
    loop {
        let mut request = server.recv()?;

        if request.method() != &tiny_http::Method::Post {
            request.respond(tiny_http::Response::empty(405))?;
            continue;
        }

        let mut body = String::new();
        request.as_reader().read_to_string(&mut body)?;
        println!("{body}");

        request.respond(tiny_http::Response::empty(204))?;
    }
}