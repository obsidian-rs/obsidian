pub trait ErrorResponse {
    fn respond_to(&self) {}
    fn status_code(&self) {}
}
