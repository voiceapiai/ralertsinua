//! The client implementation for the reqwest HTTP client, which is async
//! @borrows https://github.com/ramsayleung/rspotify/blob/master/rspotify-http/src/reqwest.rs

use std::convert::TryInto;
use std::collections::HashMap;
use std::fmt;
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;

pub const API_ALERTS_ACTIVE: &str = "v1/alerts/active.json";
pub const API_ALERTS_ACTIVE_BY_REGION_STRING: &str = "v1/iot/active_air_raid_alerts_by_oblast.json";

/// Custom enum that contains all the possible errors that may occur when using
/// [`reqwest`].
///
/// Sample usage:
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use rspotify_http::{HttpError, HttpClient, BaseHttpClient};
///
/// let client = HttpClient::default();
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
pub enum ReqwestError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Spotify with more information, which can be
    /// serialized into `rspotify_model::ApiError`.
    #[error("status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

#[derive(Debug, Clone)]
pub struct AlertsInUaClient {
    base_url: String,
    /// reqwest needs an instance of its client to perform requests.
    client: reqwest::Client,
}

impl Default for AlertsInUaClient {
    fn default() -> Self {
        let base_url = env::var("ALERTSINUA_BASE_URL").unwrap_or_default();
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()
            // building with these options cannot fail
            .unwrap();
        Self { base_url, client }
    }
}

impl AlertsInUaClient {
    async fn request<R, D>(
        &self,
        method: Method,
        url: &str,
        add_data: D,
    ) -> Result<R, ReqwestError>
    where
        R: for<'de>Deserialize<'de>,
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let url = format!("{}/{}", self.base_url, url);
        let mut request = self.client.request(method.clone(), url);

        // Enable HTTP bearer authentication.
        request = request.bearer_auth(env::var("ALERTSINUA_TOKEN").unwrap());

        // Configuring the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Finally performing the request and handling the response
        log::info!("Making request {:?}", request);
        let response = request.send().await?;

        // Making sure that the status code is OK
        if response.status().is_success() {
            response.json::<R>().await.map_err(Into::into)
            // response.text().await.map_err(Into::into)
        } else {
            Err(ReqwestError::StatusCode(response))
        }
    }

    #[cfg(test)]
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

}

/// This trait represents the interface to be implemented for an HTTP client,
/// which is kept separate from the Spotify client for cleaner code. Thus, it
/// also requires other basic traits that are needed for the Spotify client.
///
/// When a request doesn't need to pass parameters, the empty or default value
/// of the payload type should be passed, like `json!({})` or `Query::new()`.
/// This avoids using `Option<T>` because `Value` itself may be null in other
/// different ways (`Value::Null`, an empty `Value::Object`...), so this removes
/// redundancy and edge cases (a `Some(Value::Null), for example, doesn't make
/// much sense).
// #[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
pub trait BaseHttpClient: Send + Default + Clone + fmt::Debug {
    type Error;

    // This internal function should always be given an object value in JSON.
    async fn get<R>(
        &self,
        url: &str,
        payload: Option<&Query>,
    ) -> Result<R, Self::Error>
    where
        R: for<'de>Deserialize<'de>;
}

// #[cfg_attr(target_arch = "wasm32", async_impl(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_impl)]
impl BaseHttpClient for AlertsInUaClient {
    type Error = ReqwestError;

    #[inline]
    async fn get<R>(
        &self,
        url: &str,
        payload: Option<&Query<'_>>,
    ) -> Result<R, Self::Error>
    where
        R: for<'de>Deserialize<'de>,
    {
        // self.request(Method::GET, url, |req| req.query(payload))
        self.request(Method::GET, url, |r|r)
            .await
    }
}

// =============================================================================
