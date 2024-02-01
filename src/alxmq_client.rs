pub trait AlxmqClient {
    fn set_ip(mut self, ip: &str) -> Self;
    fn set_que(mut self, que_id: u8) -> Self;
}