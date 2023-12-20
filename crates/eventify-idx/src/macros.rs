#[macro_export]
macro_rules! provider {
    ($name:ident, $transport:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            inner: $transport,
        }

        impl std::ops::Deref for $name {
            type Target = $transport;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl From<$transport> for $name {
            fn from(inner: $transport) -> Self {
                Self { inner }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: [{}]", stringify!($name), stringify!($transport))
            }
        }

        impl $name {
            pub fn inner(&self) -> &$transport {
                &self.inner
            }

            pub fn with_inner(&self, inner: $transport) -> Self {
                Self { inner }
            }
        }
    };
}
