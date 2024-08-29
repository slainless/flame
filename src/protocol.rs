pub const HTTP_PROTOCOL: &str = "HTTP/1.1";

#[repr(u16)]
#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
  Continue = 100,
  SwitchingProtocols = 101,
  Processing = 102,
  EarlyHints = 103,

  OK = 200,
  Created = 201,
  Accepted = 202,
  NonAuthoritativeInformation = 203,
  NoContent = 204,
  PartialContent = 206,
  MultiStatus = 207,
  AlreadyReported = 208,
  IMUsed = 226,

  MultipleChoices = 300,
  MovedPermanently = 301,
  Found = 302,
  SeeOther = 303,
  NotModified = 304,
  UseProxy = 305,
  TemporaryRedirect = 307,
  PermanentRedirect = 308,

  BadRequest = 400,
  Unauthorized = 401,
  PaymentRequired = 402,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  NotAcceptable = 406,
  ProxyAuthenticationRequired = 407,
  RequestTimeout = 408,
  Conflict = 409,

  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502,
  ServiceUnavailable = 503,
  GatewayTimeout = 504,
  HTTPVersionNotSupported = 505,
  VariantAlsoNegotiates = 506,
  InsufficientStorage = 507,
  LoopDetected = 508,
  NotExtended = 510,
  NetworkAuthenticationRequired = 511,

  Other(u16, String)
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    if let StatusCode::Other(_, reason) = self {
      if !reason.is_empty() {
        return reason
      }
    }

    match self.to_u16() {
      100 => "Continue",
      101 => "Switching Protocols",
      102 => "Processing",
      103 => "Early Hints",
    
      200 => "OK",
      201 => "Created",
      202 => "Accepted",
      203 => "Non Authoritative Information",
      204 => "No Content",
      206 => "Partial Content",
      207 => "Multi Status",
      208 => "Already Reported",
      226 => "IM Used",
    
      300 => "Multiple Choices",
      301 => "Moved Permanently",
      302 => "Found",
      303 => "See Other",
      304 => "Not Modified",
      305 => "Use Proxy",
      307 => "Temporary Redirect",
      308 => "Permanent Redirect",
    
      400 => "Bad Request",
      401 => "Unauthorized",
      402 => "Payment Required",
      403 => "Forbidden",
      404 => "Not Found",
      405 => "Method Not Allowed",
      406 => "Not Acceptable",
      407 => "Proxy Authentication Required",
      408 => "Request Timeout",
      409 => "Conflict",
    
      500 => "Internal Server Error",
      501 => "Not Implemented",
      502 => "Bad Gateway",
      503 => "Service Unavailable",
      504 => "Gateway Timeout",
      505 => "HTTP VersionNot Supported",
      506 => "Variant Also Negotiates",
      507 => "Insufficient Storage",
      508 => "Loop Detected",
      510 => "Not Extended",
      511 => "Network Authentication Required",
      
      _ => "Other"
    }
  }

  pub fn to_u16(&self) -> u16 {
    match self {
      StatusCode::Other(code, _) => code.clone(),
      code => unsafe { *(code as *const Self as *const u16) }
    }
  }

  pub fn to_string(&self) -> String {
    let code = self.to_u16();
    code.to_string()
  }
}