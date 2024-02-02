pub mod types;

use awc::Client;
use crate::alxmq_client::AlxmqClient;
use crate::error::InternalError;
use crate::user_client::types::{RetrieveFromResponseQueRequest, RetrieveFromResponseQueResponse, ToMainQueRequest, ToMainQueResponse};

pub struct UserClient {
    ip: String,
    que_id: u8,
    client: Client
}

impl UserClient {
    pub fn new() -> UserClient {
        UserClient {
            ip: "".to_string(),
            que_id: 0,
            client: Client::default(),
        }
    }

    pub async fn send_message_to_que<T>(self, message: String) -> Result<ToMainQueResponse, InternalError> {
        let request = ToMainQueRequest {
            que_id: self.que_id.clone(),
            message,
        };
        let response_result = self.client.post(self.ip.clone() + "/to_main_que")
            .send_json(&request)
            .await;
        if response_result.is_err() {
            return Err(InternalError::new_basic_error());
        }
        let mut response = response_result.unwrap();
        if response.status().is_server_error() || response.status().is_client_error() {
            return Err(InternalError::new_basic_error());
        }
        let result = response.json().await;
        if result.is_err() {
            return Err(InternalError::new_basic_error());
        }
        Ok(result.unwrap())
    }

    pub async fn request_result_from_que(self, key: String) -> Result<RetrieveFromResponseQueResponse, InternalError> {
        let request = RetrieveFromResponseQueRequest {
            que_id: self.que_id.clone(),
            key,
        };
        let response_result = self.client.post(self.ip.clone() + "/retrieve_from_response_que")
            .send_json(&request)
            .await;
        if response_result.is_err() {
            return Err(InternalError::new_basic_error());
        }
        let mut response = response_result.unwrap();
        if response.status().is_server_error() || response.status().is_client_error() {
            return Err(InternalError::new_basic_error());
        }
        let result = response.json().await;
        if result.is_err() {
            return Err(InternalError::new_basic_error());
        }
        Ok(result.unwrap())
    }
}

impl AlxmqClient for UserClient {
    fn set_ip(mut self, ip: &str) -> Self {
        self.ip = ip.to_string();
        self
    }

    fn set_que(mut self, que_id: u8) -> Self {
        self.que_id = que_id;
        self
    }
}