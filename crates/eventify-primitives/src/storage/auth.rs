#[async_trait::async_trait]
pub trait Auth {
    /// The derived implementation should create a new connection pool with the given connection URL
    /// and immediately establish one connection.
    async fn connect(&mut self, url: &str) -> Self;

    /// The derived implementation should be using this method to create a new pool configuration
    /// and not establish connections until needed.
    fn connect_lazy(&mut self, url: &str) -> Self;
}
