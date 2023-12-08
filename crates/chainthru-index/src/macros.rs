use async_trait::async_trait;

macro_rules! define_processor_trait {
    ($trait_name:ident, $fn_suffix:ident) => {
        #[async_trait]
        pub trait $trait_name: Sync + Send {
            type Error;

            paste::item! {
                fn [< process_ $fn_suffix >](&self) -> impl std::future::Future<Output = std::result::Result<(), Self::Error>> + Send;
                fn [< stream_ $fn_suffix >](&self) -> impl std::future::Future<Output = std::result::Result<(), Self::Error>> + Send;
                fn [< stream_latest_ $fn_suffix >](&self) -> impl std::future::Future<Output = std::result::Result<(), Self::Error>> + Send;
            }
        }
    };
}

define_processor_trait!(LogProcessor, logs);
define_processor_trait!(BlockProcessor, blocks);
