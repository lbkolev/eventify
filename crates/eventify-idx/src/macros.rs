#[macro_export]
macro_rules! provider_struct {
    ($name:ident, $transport:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) inner: ethers_providers::Provider<$transport>,
        }
    };
}