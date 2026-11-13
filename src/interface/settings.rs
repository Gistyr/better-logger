// better-logger/src/interface/settings.rs

#[cfg(any(feature = "native", feature = "wasm", feature = "relay"))]
#[derive(Clone, Debug, PartialEq)]
pub enum NetworkFormat {
    PlainText,
    JsonText { field: String },
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub struct SingleNet {
    pub url: String,
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub struct MultipleNet {
    pub trace: String,
    pub debug: String,
    pub debugx: String,
    pub info: String,
    pub warn: String,
    pub error: String,
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub enum NetworkEndpointUrl {
    Single(SingleNet),
    Multiple(MultipleNet),
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Debug)]
#[non_exhaustive]
pub struct SignableRequest<'a> {
    pub url: &'a str,
    pub content_type: &'a str,
    pub body: &'a str,
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Copy)]
pub struct RequestSigner(pub fn(&SignableRequest<'_>) -> Result<Vec<(String, String)>, String>);

#[cfg(any(feature = "native", feature = "wasm"))]
impl std::fmt::Debug for RequestSigner {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return formatter.write_str("RequestSigner(..)");
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
impl PartialEq for RequestSigner {
    fn eq(&self, other: &Self) -> bool {
        return std::ptr::fn_addr_eq(self.0, other.0);
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
pub(crate) fn validated_signer_headers(
    request_signer: Option<RequestSigner>,
    signable_request: &SignableRequest<'_>,
    use_bearer_auth: bool,
) -> Result<Vec<(String, String)>, String> {
    let signer = match request_signer {
        Some(signer) => signer,
        None => return Ok(Vec::new()),
    };

    let signed_headers = match (signer.0)(signable_request) {
        Ok(headers) => headers,
        Err(error) => return Err(format!(r#"better-logger: "request_signer" failed, the log was not sent: {}"#, error)),
    };

    for (header_name, _header_value) in &signed_headers {
        let normalized_name = header_name.trim().to_ascii_lowercase();
        if normalized_name.is_empty() {
            return Err(format!(r#"better-logger: "request_signer" returned an empty header name, the log was not sent"#));
        }
        if normalized_name == "content-type" {
            return Err(format!(r#"better-logger: "request_signer" may not set "Content-Type", the log was not sent"#));
        }
        if use_bearer_auth == true && normalized_name == "authorization" {
            return Err(format!(r#"better-logger: "request_signer" may not set "Authorization" when "use_bearer_auth" is true, the log was not sent"#));
        }
    }

    return Ok(signed_headers);
}

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub struct LoggerSettings {
    pub terminal_logs: bool,
    pub terminal_log_lvl: String,
    pub wasm_logging: bool,
    pub file_logs: bool,
    pub file_log_lvl: String,
    pub log_file_path: String,
    pub network_logs: bool,
    pub network_log_lvl: String,
    pub network_endpoint_url: NetworkEndpointUrl,
    pub network_format: NetworkFormat,
    pub use_bearer_auth: bool,
    pub bearer_auth_key: String,
    pub request_signer: Option<RequestSigner>,
    pub debug_extra: bool,
    pub async_logging: bool,
}

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[cfg(any(feature = "native", feature = "wasm"))]
pub(crate) static RUNNING_SETTINGS: std::sync::OnceLock<RunningSettings> = std::sync::OnceLock::new();

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RunningSettings {
    pub(crate) terminal_logs: bool,
    pub(crate) terminal_log_lvl: String,
    pub(crate) wasm_logging: bool,
    pub(crate) file_logs: bool,
    pub(crate) file_log_lvl: String,
    pub(crate) network_logs: bool,
    pub(crate) network_log_lvl: String,
    pub(crate) network_endpoint_url: NetworkEndpointUrl,
    pub(crate) network_format: NetworkFormat,
    pub(crate) use_bearer_auth: bool,
    pub(crate) bearer_auth_key: String,
    pub(crate) request_signer: Option<RequestSigner>,
    pub(crate) debug_extra: bool,
    pub(crate) async_logging: bool,
}

#[cfg(feature = "native")]
pub(crate) static LOG_FILE: std::sync::OnceLock<std::sync::Mutex<std::fs::File>> = std::sync::OnceLock::new();

#[cfg(feature = "native")]
pub(crate) static CLIENT: std::sync::LazyLock<ureq::Agent> = std::sync::LazyLock::new(|| ureq::agent());

#[cfg(feature = "relay")]
#[derive(Clone, Debug, PartialEq)]
pub struct RelaySettings {
    pub listen_address: String,
    pub output_format: NetworkFormat,
    pub output_url: String,
    pub cors_allowed_origins: Vec<String>,
    pub actix_workers: usize,
}
