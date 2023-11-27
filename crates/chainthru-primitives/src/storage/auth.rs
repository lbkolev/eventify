#[async_trait::async_trait]
pub trait Auth {
    /// The derived implementation should create a new connection pool with the given connection URL
    /// and immediately establish one connection.
    async fn connect(url: &str) -> Self;

    // todo!("Add connect_with method");
    // The derived implementation should create a new connection pool with the given Options
    // and immediately establish one connection.
    // async fn connect_with(self, options: impl ConnectOptions) -> Self;

    /// The derived implementation should be using this method to create a new pool configuration
    /// and not establish connections until needed.
    fn connect_lazy(url: &str) -> Self;

    // todo!("Add connect_lazy_with method");
    // The derived implementation should be using this method to create a new pool configuration with the given Options
    // and not establish connections until needed.
    // fn connect_lazy_with(self, options: impl ConnectOptions) -> Self;
}
