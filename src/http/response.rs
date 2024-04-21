use super::request::{HttpMethod, Request};



#[derive(Debug)]
pub enum HttpStatusCode {
    // Informational
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // Success
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // Client Error
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl HttpStatusCode {
    pub fn to_status_code(&self) -> u16 {
        match *self {
            HttpStatusCode::Continue => 100,
            HttpStatusCode::SwitchingProtocols => 101,
            HttpStatusCode::Processing => 102,
            HttpStatusCode::EarlyHints => 103,
            HttpStatusCode::Ok => 200,
            HttpStatusCode::Created => 201,
            HttpStatusCode::Accepted => 202,
            HttpStatusCode::NonAuthoritativeInformation => 203,
            HttpStatusCode::NoContent => 204,
            HttpStatusCode::ResetContent => 205,
            HttpStatusCode::PartialContent => 206,
            HttpStatusCode::MultiStatus => 207,
            HttpStatusCode::AlreadyReported => 208,
            HttpStatusCode::IMUsed => 226,
            HttpStatusCode::MultipleChoices => 300,
            HttpStatusCode::MovedPermanently => 301,
            HttpStatusCode::Found => 302,
            HttpStatusCode::SeeOther => 303,
            HttpStatusCode::NotModified => 304,
            HttpStatusCode::UseProxy => 305,
            HttpStatusCode::TemporaryRedirect => 307,
            HttpStatusCode::PermanentRedirect => 308,
            HttpStatusCode::BadRequest => 400,
            HttpStatusCode::Unauthorized => 401,
            HttpStatusCode::PaymentRequired => 402,
            HttpStatusCode::Forbidden => 403,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::MethodNotAllowed => 405,
            HttpStatusCode::NotAcceptable => 406,
            HttpStatusCode::ProxyAuthenticationRequired => 407,
            HttpStatusCode::RequestTimeout => 408,
            HttpStatusCode::Conflict => 409,
            HttpStatusCode::Gone => 410,
            HttpStatusCode::LengthRequired => 411,
            HttpStatusCode::PreconditionFailed => 412,
            HttpStatusCode::PayloadTooLarge => 413,
            HttpStatusCode::URITooLong => 414,
            HttpStatusCode::UnsupportedMediaType => 415,
            HttpStatusCode::RangeNotSatisfiable => 416,
            HttpStatusCode::ExpectationFailed => 417,
            HttpStatusCode::MisdirectedRequest => 421,
            HttpStatusCode::UnprocessableEntity => 422,
            HttpStatusCode::Locked => 423,
            HttpStatusCode::FailedDependency => 424,
            HttpStatusCode::TooEarly => 425,
            HttpStatusCode::UpgradeRequired => 426,
            HttpStatusCode::PreconditionRequired => 428,
            HttpStatusCode::TooManyRequests => 429,
            HttpStatusCode::RequestHeaderFieldsTooLarge => 431,
            HttpStatusCode::UnavailableForLegalReasons => 451,
            HttpStatusCode::InternalServerError => 500,
            HttpStatusCode::NotImplemented => 501,
            HttpStatusCode::BadGateway => 502,
            HttpStatusCode::ServiceUnavailable => 503,
            HttpStatusCode::GatewayTimeout => 504,
            HttpStatusCode::HTTPVersionNotSupported => 505,
            HttpStatusCode::VariantAlsoNegotiates => 506,
            HttpStatusCode::InsufficientStorage => 507,
            HttpStatusCode::LoopDetected => 508,
            HttpStatusCode::NotExtended => 510,
            HttpStatusCode::NetworkAuthenticationRequired => 511,
        }
    }
}

// r#"HTTP/1.1 200 OK
// Date: Sun, 14 Apr 2024 05:22:58 GMT
// Content-Type: application/json
// Content-Length: 13
// Connection: keep-alive
// Server: nginx/1.18.0 (Ubuntu)
// Vary: Accept, Origin
// Allow: OPTIONS, GET
// X-Frame-Options: DENY
// X-Content-Type-Options: nosniff
// Referrer-Policy: same-origin
// Cross-Origin-Opener-Policy: same-origin
//
// {"test": 123}
// "#


 pub struct Response {
     status_code: HttpStatusCode,
     date: String,
     content_type: String,
     connection: String,
     content_length: Option<u64>,
     accept: String,
     very: String,
     allow: Vec<HttpMethod>,
     cross_origin_opener_policy: String,
     server: String,
     body: Vec<u8>
 }

impl Response{
    pub fn new(request: Request, status_code: HttpStatusCode, body: Vec<u8>){
    }

}
