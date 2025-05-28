
use reqwest::StatusCode;
pub mod active;
pub mod passive;



pub static ALLOWED_STATUS_CODES: &[StatusCode] = &[
    StatusCode::OK,                      // 200
    StatusCode::MOVED_PERMANENTLY,      // 301
    StatusCode::FOUND,                  // 302
    StatusCode::SEE_OTHER,              // 303
    StatusCode::TEMPORARY_REDIRECT,     // 307
    StatusCode::PERMANENT_REDIRECT,     // 308
    StatusCode::UNAUTHORIZED,           // 401
    StatusCode::FORBIDDEN,              // 403
    StatusCode::METHOD_NOT_ALLOWED,     // 405
    StatusCode::INTERNAL_SERVER_ERROR,  // 500
    StatusCode::SERVICE_UNAVAILABLE,    // 503
];
