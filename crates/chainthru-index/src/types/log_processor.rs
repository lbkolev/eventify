pub trait LogProcessor: Sync + Send {
    type Error;

    async fn process_logs(&self) -> std::result::Result<(), Self::Error>;
    async fn stream_logs(&self) -> std::result::Result<(), Self::Error>;
    async fn stream_latest_logs(&self) -> std::result::Result<(), Self::Error>;
}
