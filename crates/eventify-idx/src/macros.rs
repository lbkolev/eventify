#[macro_export]
macro_rules! provider {
    ($name:ident, $transport:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub inner: $transport,
        }
    };
}
