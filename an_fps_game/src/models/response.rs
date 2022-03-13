#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub success: bool,
    pub message: String,
}
