use thiserror::Error;

/// Arango Http error based on the response http code
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ArangoHttpError {
    /// 400 - ERROR_HTTP_BAD_PARAMETER
    ///
    /// Will be raised when the HTTP request does not fulfill the requirements.
    #[error("400 - ERROR_HTTP_BAD_PARAMETER")]
    BadParameter,
    /// 401 - ERROR_HTTP_UNAUTHORIZED
    ///
    /// Will be raised when authorization is required but the user is not authorized.
    #[error("401 - ERROR_HTTP_UNAUTHORIZED")]
    Unauthorized,
    /// 403 - ERROR_HTTP_FORBIDDEN
    ///
    /// Will be raised when the operation is forbidden.
    #[error("403 - ERROR_HTTP_FORBIDDEN")]
    Forbidden,
    /// 404 - ERROR_HTTP_NOT_FOUND
    ///
    /// Will be raised when an URI is unknown or a database element is not found.
    #[error("404 - ERROR_HTTP_NOT_FOUND")]
    NotFound,
    /// 405 - ERROR_HTTP_METHOD_NOT_ALLOWED
    ///
    /// Will be raised when an unsupported HTTP method is used for an operation.
    #[error("405 - ERROR_HTTP_METHOD_NOT_ALLOWED")]
    MethodNotAllowed,
    /// 406 - ERROR_HTTP_NOT_ACCEPTABLE
    ///
    /// Will be raised when an unsupported HTTP content type is used for an operation
    #[error("406 - ERROR_HTTP_NOT_ACCEPTABLE")]
    NotAcceptable,
    /// 409 - ERROR_HTTP_CONFLICT
    ///
    /// Will be raised when conflict occured (index unique constraint for example)
    #[error("409 - ERROR_HTTP_CONFLICT")]
    Conflict,
    /// 412 - ERROR_HTTP_PRECONDITION_FAILED
    ///
    /// Will be raised when a precondition for an HTTP request is not met.
    #[error("412 - ERROR_HTTP_PRECONDITION_FAILED")]
    PreconditionFailed,
    /// 500 - ERROR_HTTP_SERVER_ERROR
    ///
    /// Will be raised when an internal server is encountered.
    #[error("500 - ERROR_HTTP_SERVER_ERROR")]
    ServerError,
    /// 503 - ERROR_HTTP_SERVICE_UNAVAILABLE
    ///
    /// Will be raised when a service is temporarily unavailable.
    #[error("503 - ERROR_HTTP_SERVICE_UNAVAILABLE")]
    ServiceUnavailable,
    /// 504 - ERROR_HTTP_GATEWAY_TIMEOUT
    ///
    /// Will be raised when a service contacted by `ArangoDB` does not respond in a timely manner.
    #[error("504 - ERROR_HTTP_GATEWAY_TIMEOUT")]
    GatewayTimeout,
    /// 600 - ERROR_HTTP_CORRUPTED_JSON
    ///
    /// Will be raised when a string representation of a JSON object is corrupt.
    #[error("600 - ERROR_HTTP_CORRUPTED_JSON")]
    CorruptedJson,
    /// 601 - ERROR_HTTP_SUPERFLUOUS_SUFFICES
    ///
    /// Will be raised when the URL contains superfluous suffices.
    #[error("601 - ERROR_HTTP_SUPERFLUOUS_SUFFICES")]
    SuperfluousSuffices,
    /// Unknown Http error, happens when the HTTP code is not handled
    #[error("Unhandled Arango Http Error: `{0}`")]
    UnknownError(u16),
}
impl ArangoHttpError {
    pub(crate) const fn from_code(code: u16) -> Self {
        match code {
            400 => Self::BadParameter,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            409 => Self::Conflict,
            412 => Self::PreconditionFailed,
            500 => Self::ServerError,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            600 => Self::CorruptedJson,
            601 => Self::SuperfluousSuffices,
            _ => Self::UnknownError(code),
        }
    }

    /// The HTTP code matching the enum variant
    pub const fn http_code(&self) -> u16 {
        match self {
            ArangoHttpError::BadParameter => 400,
            ArangoHttpError::Unauthorized => 401,
            ArangoHttpError::Forbidden => 403,
            ArangoHttpError::NotFound => 404,
            ArangoHttpError::MethodNotAllowed => 405,
            ArangoHttpError::NotAcceptable => 406,
            ArangoHttpError::Conflict => 409,
            ArangoHttpError::PreconditionFailed => 412,
            ArangoHttpError::ServerError => 500,
            ArangoHttpError::ServiceUnavailable => 503,
            ArangoHttpError::GatewayTimeout => 504,
            ArangoHttpError::CorruptedJson => 600,
            ArangoHttpError::SuperfluousSuffices => 601,
            ArangoHttpError::UnknownError(code) => *code,
        }
    }
}
