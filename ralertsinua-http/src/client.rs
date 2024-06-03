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
use std::{collections::HashMap, sync::Arc};
use std::{fmt, sync::RwLock};
use time::OffsetDateTime;

#[cfg(feature = "cache")]
use crate::cache::*;
use crate::error::*;

type Query<'a> = HashMap<&'a str, &'a str>;
type Result<T> = miette::Result<T, ApiError>;

pub const API_BASE_URL: &str = "https://api.alerts.in.ua";
pub const API_VERSION: &str = "/v1";
pub const API_CACHE_SIZE: usize = 1000;
#[rustfmt::skip]
const CACHE_ENABLED_STR: &str = if cfg!(feature = "cache") { "enabled" } else { "disabled" };

const PKG: &str = env!("CARGO_PKG_NAME");
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct AlertsInUaClient {
    base_url: String,
    token: String,
    client: Client,
    meta_data: Arc<RwLock<AlertsMeta>>,
    #[cfg(feature = "cache")]
    cache_manager: Arc<dyn CacheManagerSync>,
}

impl std::fmt::Debug for AlertsInUaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlertsInUaClient {{ base_url: {}, token: {}, client: {:?}, meta_data: {:?}, cache_manager: {:?} }}", self.base_url, self.token, self.client, self.meta_data, "CACacheManager")
    }
}

impl AlertsInUaClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let base_url = base_url.into();
        let token = token.into();
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(USER_AGENT)
            .build()
            // building with these options cannot fail
            .unwrap();
        let meta_data = Arc::new(RwLock::new(AlertsMeta::default()));
        #[cfg(feature = "cache")]
        let cache_manager = Arc::new(CacheManagerQuick::new(API_CACHE_SIZE));

        log::debug!(target: PKG, "Caching is {}", CACHE_ENABLED_STR);

        Self {
            base_url,
            token,
            client,
            meta_data,
            #[cfg(feature = "cache")]
            cache_manager,
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
        let mut last_modified = self
            .meta_data
            .read()
            .unwrap()
            .clone()
            .get_last_updated_at_formatted();
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

        if cfg!(feature = "cache") {
            if let Some(CacheEntry(bytes)) = self.cache_manager.get(&url)? {
                cached_data = bytes;
            }
            // Here we set the If-Modified-Since header from the last_modified
            headers.insert(
                "If-Modified-Since",
                last_modified.parse().map_err(http::Error::from)?,
            );
        }

        req = req.headers(headers);
        // Configuring the request for the specific type (get/post/put/delete)
        req = add_data(req);
        // Finally performing the request and handling the response
        log::trace!(target: PKG, "Request {:?}", req);
        let res: Response = req.send().await.inspect_err(|e| {
            log::error!(target: PKG,  "Error making request: {:?}", e);
        })?;
        log::trace!(target: PKG, "Response {:?}", res);
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

        last_modified = format!("{:?}", res.headers().get("Last-Modified").unwrap())
            .trim_matches('"')
            .to_string();
        #[allow(unused)]
        let last_modified_datetime = self
            .meta_data
            .write()
            .unwrap()
            .set_last_updated_at(&last_modified)
            .map_err(|e| {
                log::error!("Error updating meta data: {:?}", e);
                ApiError::Internal
            })?;
        // -------------------------------------------------------------
        let data: Bytes = match res.status() {
            #[cfg(feature = "cache")]
            StatusCode::NOT_MODIFIED => {
                log::trace!(target: PKG, "Response status '304 Not Modified', return cached data");
                cached_data
            }
            _ => {
                let bytes = res.bytes().await?;
                if cfg!(feature = "cache") {
                    // Save the data to the cache
                    self.cache_manager.put(&url, bytes.clone())?;
                }

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

    async fn get_last_modified(&self) -> Result<OffsetDateTime>;
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

    async fn get_last_modified(&self) -> Result<OffsetDateTime> {
        #[allow(clippy::clone_on_copy)]
        Ok(self.meta_data.read().unwrap().get_last_updated_at().clone())
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

    #[test]
    fn test_get_api_url() {
        let client = AlertsInUaClient::new("https://api.alerts.in.ua", "token");
        let url = client.get_api_url("/alerts/active.json");
        assert_eq!(url, "https://api.alerts.in.ua/v1/alerts/active.json");
    }

    #[tokio::test]
    async fn test_get_active_alerts() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token");
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
    async fn test_get_alerts_history() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token");
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

        let result = client.get_alerts_history(&1, "week_ago").await?;

        mock.assert();
        assert_eq!(result, expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_air_raid_alert_statuses_by_location() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token");
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

    // test get_last_modified
    #[tokio::test]
    async fn test_update_last_modified_from_response() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url().as_str(), "token");
        let mock = server
                .mock(
                    "GET",
                    mockito::Matcher::Any,
                )
                .with_header("Last-Modified", "Tue, 14 May 2024 18:18:18 GMT")
                .with_body(r#"{"alerts":[],"disclaimer":"","meta":{"last_updated_at":"2024/05/14 18:18:18 +0000"}}"#)
                .create_async()
                .await;
        let result: Alerts = client.get_alerts_history(&1, "week_ago").await?;

        let last_modified = client.get_last_modified().await?;

        mock.assert();
        // check last_modified updated
        assert_eq!(last_modified.minute(), 18);
        // check last_modified equal to response date with custom formatting
        assert_eq!(result.get_last_updated_at(), last_modified);

        Ok(())
    }
}
