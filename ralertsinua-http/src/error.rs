/// Custom enum that contains all the possible errors that may occur when using
/// [`reqwest`].
///
/// Sample usage:
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use http::{HttpError, AlertsInUaClient, BaseHttpClient};
///
/// let client = AlertsInUaClient::default();
/// let response = client.get("wrongurl", None, &Default::default()).await;
/// match response {
///     Ok(data) => println!("request succeeded: {:?}", data),
///     Err(HttpError::Client(e)) => eprintln!("request failed: {}", e),
///     Err(HttpError::StatusCode(response)) => {
///         let code = response.status().as_u16();
///         match response.json::<rspotify_model::ApiError>().await {
///             Ok(api_error) => eprintln!("status code {}: {:?}", code, api_error),
///             Err(_) => eprintln!("status code {}", code),
///         }
///     },
/// }
/// # }
/// ```
#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[diagnostic(code(ralertsinua_http::client))]
pub enum ApiError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error(transparent)]
    // #[diagnostic(code(my_lib::bad_code))]
    Unknown(#[from] reqwest::Error),

    #[error(transparent)]
    MiddleWareError(#[from] reqwest_middleware::Error),

    #[error("API Error: Invalid token")]
    InvalidToken,

    #[error("API Error: Unauthorized: {0}")]
    #[diagnostic(help("most likely token is invalid or missing\n check you've provided it via environment variable 'ALERTSINUA_TOKEN' or as a parameter '--token'"))]
    UnauthorizedError(reqwest::Error),

    #[error("API Error: Rate limit exceeded")]
    RateLimitError,

    #[error("API Error: Internal server error")]
    InternalServerError,

    #[error("API Error: Forbidden")]
    ForbiddenError,

    #[error("API Error: Invalid parameter")]
    InvalidParameterException,

    #[error("API Error: Invalid URL: {0}")]
    InvalidURL(reqwest::Error),

    #[error("API Error: Internal error")]
    Internal,
}
