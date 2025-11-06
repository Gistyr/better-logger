#![cfg(feature = "relay")]

use actix_cors::Cors;
use actix_web::{
    http::header::CONTENT_TYPE,
    http::Method,
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use awc::Client;
use serde_json::json;

use super::settings::{RelaySettings, NetworkFormat};

fn read_plaintext_line(req: &HttpRequest, body: &[u8]) -> Result<String, (u16, String)> {
    let ct_ok = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .map(|ct| ct.to_ascii_lowercase().starts_with("text/plain"))
        .unwrap_or(false);

    if !ct_ok {
        return Err((415, "unsupported media type (expected text/plain)".into()));
    }

    String::from_utf8(body.to_vec())
        .map_err(|e| (400, format!("invalid UTF-8 body: {e}")))
}

async fn forward_line(client: &Client, url: &str, fmt: &NetworkFormat, line: &str)
    -> Result<(), String>
{
    match fmt {
        NetworkFormat::PlainText => {
            client.post(url)
                .insert_header(("Content-Type", "text/plain; charset=utf-8"))
                .send_body(line.to_owned())
                .await
                .map_err(|e| format!("forward(PlainText) failed: {e}"))?
                .body()
                .await
                .map_err(|e| format!("forward body read failed: {e}"))?;
            Ok(())
        }
        NetworkFormat::JsonText { field } => {
            let payload = json!({ field: line }).to_string();
            client.post(url)
                .insert_header(("Content-Type", "application/json; charset=utf-8"))
                .send_body(payload)
                .await
                .map_err(|e| format!("forward(JsonText) failed: {e}"))?
                .body()
                .await
                .map_err(|e| format!("forward body read failed: {e}"))?;
            Ok(())
        }
    }
}

async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn relay_post(
    req: HttpRequest,
    body: web::Bytes,
    cfg: web::Data<RelaySettings>,
    client: web::Data<Client>,
) -> actix_web::Result<HttpResponse> {
    let line = match read_plaintext_line(&req, &body) {
        Ok(s) => s,
        Err((code, msg)) => {
            return Ok(
                HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap())
                    .body(msg)
            );
        }
    };

    match forward_line(&client, &cfg.output_url, &cfg.output_format, &line).await {
        Ok(_) => Ok(HttpResponse::Accepted().finish()),
        Err(e) => Ok(HttpResponse::BadGateway().body(e)), 
    }
}

pub async fn start(config: RelaySettings) -> Result<(), String> {
    let listen = config.listen_address.clone();
    let cors_cfg = config.cors_allowed_origins.clone();

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["POST", "OPTIONS", "GET"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        if cors_cfg.iter().any(|o| o == "*") {
            cors = cors.allow_any_origin();
        } else {
            for origin in &cors_cfg {
                cors = cors.allowed_origin(origin);
            }
        }

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(Client::default()))
            .route("/healthz", web::get().to(healthz))
            .route("/", web::post().to(relay_post))
            .route("/", web::route().method(Method::OPTIONS).to(|| async {
                HttpResponse::NoContent().finish()
            }))
    })
    .bind(listen.clone())
    .map_err(|e| format!("bind {} failed: {}", listen, e))?
    .run()
    .await
    .map_err(|e| format!("server error: {e}"))
}