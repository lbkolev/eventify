macro_rules! define_processor_trait {
    ($trait_name:ident, $fn_suffix:ident) => {
        #[async_trait::async_trait]
        pub trait $trait_name: Sync + Send {
            type Error;

            paste::item! {
                async fn [< process_ $fn_suffix >](&self) -> std::result::Result<(), Self::Error>;
                async fn [< stream_ $fn_suffix >](&self) -> std::result::Result<(), Self::Error>;
                async fn [< stream_latest_ $fn_suffix >](&self) -> std::result::Result<(), Self::Error>;
            }
        }
    };
}

define_processor_trait!(LogProcessor, logs);
define_processor_trait!(BlockProcessor, blocks);
