use crate::client::StreamChat;
use crate::models::*;

impl StreamChat {
    pub async fn list_devices(
        &self,
        user_id: String,
    ) -> Result<ListDevicesResponse, crate::models::ApiError> {
        Ok(self.get("devices", Some(&[("user_id", user_id)])).await?)
    }

    pub async fn delete_device(
        &self,
        user_id: String,
        device_id: String,
    ) -> Result<Response, ApiError> {
        Ok(self
            .delete("devices", Some(&[("user_id", user_id), ("id", device_id)]))
            .await?)
    }

    pub async fn create_device(&self, device: &CreateDeviceRequest) -> Result<Response, ApiError> {
        Ok(self.post("devices", device).await?)
    }
}
