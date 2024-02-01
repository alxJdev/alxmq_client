mod types;

use awc::Client;
use crate::alxmq_client::AlxmqClient;

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

    pub async fn send_message_to_que(self) {

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