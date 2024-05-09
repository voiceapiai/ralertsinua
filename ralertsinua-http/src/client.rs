//! The client implementation for the reqwest HTTP client, which is async
//! @borrows https://github.com/ramsayleung/rspotify/blob/master/rspotify-http/src/reqwest.rs

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, RequestBuilder, StatusCode,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use crate::ApiError;
use ralertsinua_models::*;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
type Result<T> = std::result::Result<T, ApiError>;

pub const API_BASE_URL: &str = "https://api.alerts.in.ua";
pub const API_VERSION: &str = "/v1";
// Name your user agent after your app?
const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    // "/",
    // env!("VERGEN_CARGO_TARGET_TRIPLE"),
);

#[derive(Debug, Clone)]
pub struct AlertsInUaClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl AlertsInUaClient {
    #[rustfmt::skip]
    pub fn new<U, T>(base_url: U, token: T) -> Self where U: Into<String>, T: Into<String>,
    {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .user_agent(APP_USER_AGENT)
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert("accept", HeaderValue::from_static("value"));
                headers
            })
            .build()
            // building with these options cannot fail
            .unwrap();
        Self {
            base_url: base_url.into(),
            token: token.into(),
            client,
        }
    }
}

impl AlertsInUaClient {
    fn get_api_url(&self, url: &str) -> String {
        let version = API_VERSION;
        let base_url = self.base_url.clone();
        // if !base_url.ends_with('/') { base_url.push('/'); }
        base_url + version + url
    }

    async fn request<R, D>(&self, method: Method, url: &str, add_data: D) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        // Build full URL
        let url = self.get_api_url(url);
        let mut request = self.client.request(method.clone(), url);
        // Enable HTTP bearer authentication.
        request = request.bearer_auth(&self.token);

        // Configuring the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Finally performing the request and handling the response
        // log::info!("Making request {:?}", request);
        let response = request.send().await?;

        // Making sure that the status code is OK

        match response.error_for_status() {
            Ok(res) => res.json::<R>().await.map_err(Into::into),
            Err(err) => match err.status() {
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
            },
        }
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
/// TODO: If-Modified-Since + Last-Modified for caching
pub trait BaseHttpClient: Send + Clone + fmt::Debug {
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
pub trait AlertsInUaApi: Sync + Send + core::fmt::Debug {
    #[allow(async_fn_in_trait)]
    async fn get_active_alerts(&self) -> Result<Alerts>;

    #[allow(async_fn_in_trait)] // 'week_ago'
    async fn get_alerts_history(&self, location_aid: &i8, period: &str) -> Result<Alerts>;

    #[allow(async_fn_in_trait)] // 'week_ago'
    async fn get_air_raid_alert_status(&self, location_aid: &i8) -> Result<String>;

    #[allow(async_fn_in_trait)]
    async fn get_air_raid_alert_statuses_by_location(
        &self,
    ) -> Result<AirRaidAlertOblastStatuses>;
}

#[async_trait]
impl AlertsInUaApi for AlertsInUaClient {
    #[inline]
    async fn get_active_alerts(&self) -> Result<Alerts> {
        let url = "/alerts/active.json";
        self.get(url, &Query::default()).await
    }

    #[inline]
    async fn get_alerts_history(&self, location_aid: &i8, period: &str) -> Result<Alerts> {
        let url = format!("/locations/{}/alerts/{}.json", location_aid, period);
        self.get(&url, &Query::default()).await
    }

    #[inline]
    async fn get_air_raid_alert_status(&self, location_aid: &i8) -> Result<String> {
        let url = format!("/iot/active_air_raid_alerts/{}.json", location_aid);
        self.get(&url, &Query::default()).await
    }

    #[inline]
    async fn get_air_raid_alert_statuses_by_location(
        &self,
    ) -> Result<AirRaidAlertOblastStatuses> {
        let url = "/iot/active_air_raid_alerts_by_oblast.json";
        let data: String = self.get(url, &Query::default()).await?;
        let result = AirRaidAlertOblastStatuses::new(data, Some(true));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use mockito::Server as MockServer;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_trait() {
        let api_client: Arc<dyn AlertsInUaApi> = Arc::new(AlertsInUaClient::new("", ""));
        println!("{:?}", api_client);
    }

    #[test]
    fn test_get_api_url() {
        let client = AlertsInUaClient::new("https://api.alerts.in.ua", "token");
        let url = client.get_api_url("/alerts/active.json");
        assert_eq!(url, "https://api.alerts.in.ua/v1/alerts/active.json");
    }

    #[tokio::test]
    async fn test_get_active_alerts() -> Result<()> {
        let mut server = MockServer::new_async().await;
        let client = AlertsInUaClient::new(server.url(), "token");
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
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
        let client = AlertsInUaClient::new(server.url(), "token");
        let mock = server
            .mock(
                "GET",
                mockito::Matcher::Any, /* API_ALERTS_ACTIVE_BY_REGION_STRING */
            )
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
