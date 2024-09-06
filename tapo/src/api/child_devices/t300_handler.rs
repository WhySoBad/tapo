use std::sync::Arc;

use tokio::sync::RwLock;

use crate::api::ApiClient;
use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, GetTriggerLogsParams, TapoParams, TapoRequest};
use crate::responses::{DecodableResultExt, T300Result};
use crate::responses::{T300Log, TriggerLogsResult};

/// Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices.
pub struct T300Handler {
    client: Arc<RwLock<ApiClient>>,
    device_id: String,
}

impl T300Handler {
    pub(crate) fn new(client: Arc<RwLock<ApiClient>>, device_id: String) -> Self {
        Self { client, device_id }
    }

    /// Returns *device info* as [`T300Result`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<T300Result, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child::<T300Result>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns a list of trigger logs.
    ///
    /// # Arguments
    ///
    /// * `page_size` - the maximum number of log items to return
    /// * `start_id` - the log item `id` from which to start returning results in reverse chronological order (newest first)
    ///
    /// Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.
    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> Result<TriggerLogsResult<T300Log>, Error> {
        let child_params = GetTriggerLogsParams::new(page_size, start_id);
        let child_request = TapoRequest::GetTriggerLogs(Box::new(TapoParams::new(child_params)));

        self.client
            .read()
            .await
            .control_child::<TriggerLogsResult<T300Log>>(self.device_id.clone(), child_request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }
}
