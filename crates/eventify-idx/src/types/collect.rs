use alloy_primitives::BlockNumber;
use eventify_primitives::Criteria;

/// `Collect` Trait
///
/// An asynchronous trait designed for processing various types of data.
/// Implementers of this trait typically handle tasks such as fetching,
/// parsing, and storing data asynchronously. The trait provides a flexible
/// interface for different kinds of data processing activities, allowing
/// implementers to define the specifics of these activities.
// TODO: implement it for Collector
#[async_trait::async_trait]
pub trait Collect<T, E>
where
    T: Into<Criteria>,
    E: std::error::Error + Send + Sync,
{
    //async fn collect(&self) -> Result<(), E>;

    async fn process_logs(&self, c: T) -> Result<(), E>;
    async fn process_block(&self, b: BlockNumber) -> Result<(), E>;
    async fn process_blocks(&self, from: BlockNumber, to: BlockNumber) -> Result<(), E>;
    async fn process_transactions(&self, b: BlockNumber) -> Result<(), E>;
    async fn process_transactions_from_range(
        &self,
        from: BlockNumber,
        to: BlockNumber,
    ) -> Result<(), E>;
}
