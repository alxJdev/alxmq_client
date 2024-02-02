pub trait AlxmqClient {
    fn set_ip(self, ip: &str) -> Self;
    fn set_que(self, que_id: u8) -> Self;
}