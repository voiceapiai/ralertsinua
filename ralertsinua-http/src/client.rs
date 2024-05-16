//! The client implementation for the reqwest HTTP client, which is async
//! @borrows https://github.com/ramsayleung/rspotify/blob/master/rspotify-http/src/reqwest.rs

use async_trait::async_trait;
use bytes::Bytes;
use ralertsinua_models::*;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Method, RequestBuilder, Response, StatusCode,
};
use serde::Deserialize;
use std::fmt;
use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "cache")]
use crate::{cache::*, error::*};

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;

type Result<T> = miette::Result<T, ApiError>;

pub const API_BASE_URL: &str = "https://api.alerts.in.ua";
pub const API_VERSION: &str = "/v1";
pub const API_CACHE_SIZE: usize = 1000;
// Name your user agent after your app?
const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    // "/",
    // env!("VERGEN_CARGO_TARGET_TRIPLE"),
);

pub struct AlertsInUaClient {
    base_url: String,
    token: String,
    client: Client,
    #[allow(unused)]
    cache_manager: Arc<dyn CacheManagerSync>,
}

impl std::fmt::Debug for AlertsInUaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlertsInUaClient {{ base_url: {}, token: {}, client: {:?}, cache_manager: {:?} }}", self.base_url, self.token, self.client, "CACacheManager")
    }
}

impl AlertsInUaClient {
    #[cfg(not(feature = "cache"))]
    pub fn new(base_url: &str, token: &str) -> Self {
        let base_url = base_url.into();
        let token = token.into();
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Failed to build reqwest client");

        // building with these options cannot fail
        Self {
            base_url,
            token,
            client,
            // cache_manager: manager.clone(),
        }
    }

    #[cfg(feature = "cache")]
    pub fn new(
        base_url: &str,
        token: &str,
        cache_manager: Option<Arc<dyn CacheManagerSync>>,
    ) -> Self {
        let base_url = base_url.into();
        let token = token.into();
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(APP_USER_AGENT)
            .build()
            // building with these options cannot fail
            .unwrap();

        let cache_manager = cache_manager
            .unwrap_or_else(|| Arc::new(CacheManagerQuick::new(API_CACHE_SIZE)));

        Self {
            base_url,
            token,
            client,
            cache_manager: cache_manager.clone(),
        }
    }
}

impl AlertsInUaClient {
    fn get_api_url(&self, url: &str) -> String {
        format!("{}{}{}", self.base_url, API_VERSION, url)
    }

    async fn request<R, D>(&self, method: Method, url: &str, add_data: D) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut last_modified = String::new();
        let mut cached_data: Bytes = Bytes::new();
        // Build full URL
        let url = self.get_api_url(url);
        let mut req = self.client.request(method.clone(), &url);
        // Enable HTTP bearer authentication.
        req = req.bearer_auth(&self.token);
        // Get last_modified from cache
        let mut headers = HeaderMap::new();
        // Set the headers
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        if let Some(CacheEntry(bytes, lm)) = self.cache_manager.get(&url)? {
            last_modified = lm;
            cached_data = bytes;
        }
        // Here we set the If-Modified-Since header from the last_modified
        headers.insert(
            "If-Modified-Since",
            last_modified.parse().map_err(http::Error::from)?,
        );
        req = req.headers(headers);
        // Configuring the request for the specific type (get/post/put/delete)
        req = add_data(req);

        // Finally performing the request and handling the response
        log::trace!(target: APP_USER_AGENT, "Request {:?}", req);
        let res: Response = req.send().await.inspect_err(|e| {
            log::error!("Error making request: {:?}", e);
        })?;
        log::trace!(target: APP_USER_AGENT, "Response {:?}", res);
        // Making sure that the status code is OK
        if let Err(err) = res.error_for_status_ref() {
            let err = match err.status() {
                Some(StatusCode::BAD_REQUEST) => Err(ApiError::InvalidParameterException),
                Some(StatusCode::UNAUTHORIZED) => Err(ApiError::UnauthorizedError(err)),
                Some(StatusCode::FORBIDDEN) => Err(ApiError::InvalidParameterException),
                Some(StatusCode::METHOD_NOT_ALLOWED) | Some(StatusCode::NOT_FOUND) => {
                    Err(ApiError::InvalidURL(err))
                }
                Some(StatusCode::TOO_MANY_REQUESTS) => Err(ApiError::RateLimitError),
                Some(StatusCode::INTERNAL_SERVER_ERROR) => {
                    Err(ApiError::InternalServerError)
                }
                _ => Err(ApiError::Unknown(err)),
            };

            return err;
        }

        last_modified = format!("{:?}", res.headers().get("Last-Modified").unwrap());
        // -------------------------------------------------------------
        let data: Bytes = match res.status() {
            StatusCode::NOT_MODIFIED => {
                log::trace!(target: APP_USER_AGENT, "Response status was not modified, using cached data");
                cached_data
            }
            _ => {
                let bytes = res.bytes().await?;
                // Save the data to the cache
                self.cache_manager
                    .put(&url, &last_modified, bytes.clone())
                    .inspect_err(|e| {
                        log::error!("Error writing to cache: {:?}", e);
                    })?;

                bytes
            }
        };

        // Return deserialized data
        Ok(serde_json::from_slice(&data)?)
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
pub trait BaseHttpClient: Send + fmt::Debug {
    // This internal function should always be given an object value in JSON.
    #[allow(async_fn_in_trait)]
    async fn get<R>(&self, url: &str, payload: &Query) -> Result<R>
    where
        R: for<'de> Deserialize<'de>;
}

impl BaseHttpClient for AlertsInUaClient {
    #[inline]
    async fn get<R>(&self, url: &str, _payload: &Query<'_>) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        self.request(Method::GET, url, |r| r).await
    }
}

/// The API for the AlertsInUaClient
#[async_trait]
pub trait AlertsInUaApi: fmt::Debug {
    async fn get_active_alerts(&self) -> Result<Alerts>;

    async fn get_alerts_history(&self, location_aid: &i8, period: &str) -> Result<Alerts>;

    async fn get_air_raid_alert_status(&self, location_aid: &i8) -> Result<String>;

    async fn get_air_raid_alert_statuses_by_location(
        &self,
    ) -> Result<AirRaidAlertOblastStatuses>;
}

#[async_trait]
impl AlertsInUaApi for AlertsInUaClient {
    async fn get_active_alerts(&self) -> Result<Alerts> {
        let url = "/alerts/active.json";
        self.get(url, &Query::default()).await
    }

    async fn get_alerts_history(&self, location_aid: &i8, period: &str) -> Result<Alerts> {
        let url = format!("/locations/{}/alerts/{}.json", location_aid, period);
        self.get(&url, &Query::default()).await
    }

    async fn get_air_raid_alert_status(&self, location_aid: &i8) -> Result<String> {
        let url = format!("/iot/active_air_raid_alerts/{}.json", location_aid);
        self.get(&url, &Query::default()).await
    }

    async fn get_air_raid_alert_statuses_by_location(
        &self,
    ) -> Result<AirRaidAlertOblastStatuses> {
        let url = "/iot/active_air_raid_alerts_by_oblast.json";
        let data: String = self.get(url, &Query::default()).await?;
        let result = AirRaidAlertOblastStatuses::new(data, Some(true));
        Ok(result)
    }
}

// The existence of this function makes the compiler catch if the Buf
// trait is "object-safe" or not.
fn _assert_trait_object(_: &dyn AlertsInUaApi) {}

#[cfg(test)]
mod tests {

    use super::*;
    #[allow(unused_imports)]
    use mockall::predicate::*;
    use mockito::Server as MockServer;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_trait() {
        let api_client: Arc<dyn AlertsInUaApi> =
            Arc::new(AlertsInUaClient::new("", "", None));
        println!("{:?}", api_client);
    }

    /* #[tokio::test]
    async fn test_get_last_modified() {
        let client = AlertsInUaClient::new("https://api.alerts.in.ua", "token", None);
        let result = client.get_last_modified().await;
        assert!(result.is_ok());
    } */

    #[test]
    fn test_get_api_url() {
        let client = AlertsInUaClient::new("https://api.alerts.in.ua", "token", None);
        let url = client.get_api_url("/alerts/active.json");
        assert_eq!(url, "https://api.alerts.in.ua/v1/alerts/active.json");
    }

    #[tokio::test]
    async fn test_get_active_alerts() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token", None);
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
            .with_header("Last-Modified", "Tue, 14 May 2024 18:18:18 GMT")
            .with_body(r#"{"alerts":[],"disclaimer":"","meta":{"last_updated_at":"2024/05/06 10:02:45 +0000"}}"#)
            .create_async()
            .await;
        let expected_response: Alerts =
            serde_json::from_value(json!({"alerts":[],"disclaimer":"","meta":{"last_updated_at":"2024/05/06 10:02:45 +0000"}})).unwrap();

        let result = client.get_active_alerts().await?;

        mock.assert();
        assert_eq!(result, expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_air_raid_alert_statuses_by_location() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token", None);
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
            .with_header("Last-Modified", "Tue, 14 May 2024 18:18:18 GMT")
            .with_body(r#""ANNAANNANNNPANANANNNNAANNNN""#)
            .create_async()
            .await;

        let _result = client.get_air_raid_alert_statuses_by_location().await?;

        mock.assert();
        // FIXME:
        // assert_eq!(&*result, "ANNAANNANNNPANANANNNNAANNNN");

        Ok(())
    }
}
