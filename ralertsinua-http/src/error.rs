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
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("API Error: {0}")]
    Unknown(#[from] reqwest::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message with more information, which can be
    /// serialized into `ApiError`.
    #[error("API Error: Status code {}", reqwest::Response::status(.0))]
    Unknown2(reqwest::Response),

    #[error("API Error: Invalid token")]
    InvalidToken,

    #[error("API Error: Unauthorized: {0}")]
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
}
