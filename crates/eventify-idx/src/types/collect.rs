/// `Collect` Trait
///
/// An asynchronous trait designed for processing various types of data.
/// Implementers of this trait typically handle tasks such as fetching,
/// parsing, and storing data asynchronously. The trait provides a flexible
/// interface for different kinds of data processing activities, allowing
/// implementers to define the specifics of these activities.
// TODO: implement it for Collector
#[async_trait::async_trait]
pub trait Collect {
    type Error;

    async fn collect(&self) -> Result<(), Self::Error>;

    async fn process_logs(&self) -> Result<(), Self::Error>;
    async fn process_blocks(&self) -> Result<(), Self::Error>;
    async fn process_transactions(&self) -> Result<(), Self::Error>;
}
