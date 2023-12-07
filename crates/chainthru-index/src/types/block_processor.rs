pub trait BlockProcessor: Sync + Send {
    type Error;

    async fn process_blocks(&self) -> std::result::Result<(), Self::Error>;
    async fn stream_blocks(&self) -> std::result::Result<(), Self::Error>;
    async fn stream_latest_blocks(&self) -> std::result::Result<(), Self::Error>;
}
