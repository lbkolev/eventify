use async_trait::async_trait;

/// The `Process` trait defines asynchronous operations for processing different types of data.
/// It's generic over a type `T`, allowing flexibility in the type of data being processed.
///
/// This trait is ideal for scenarios where data needs to be processed, streamed, or monitored in real-time.
#[async_trait]
pub trait Process<T> {
    type Error;

    /// Asynchronously processes data of type `T`.
    ///
    /// This method should contain the core logic for processing the data.
    /// The specifics of what processing entails (e.g., fetching, parsing, storing)
    /// are determined by the implementing type.
    async fn process(&self) -> Result<(), Self::Error>;

    /// Asynchronously streams data of type `T`.
    ///
    /// This method should provide the functionality to continuously stream data.
    /// The nature of the streaming (e.g., real-time, batch processing)
    /// depends on the implementing type.
    async fn stream(&self) -> Result<(), Self::Error>;

    /// Asynchronously streams the latest available data of type `T`.
    ///
    /// This method should specifically focus on streaming the most recent data.
    /// It's useful in scenarios where the latest information is critical, such as
    /// real-time monitoring or alerting systems.
    async fn stream_latest(&self) -> Result<(), Self::Error>;
}
