use async_trait::async_trait;

//macro_rules! define_processor_trait {
//    ($trait_name:ident, $fn_suffix:ident, $($macro_attrs:tt)*) => {
//        $($macro_attrs)*
//        pub trait $trait_name: Sync + Send {
//            type Error;
//
//            paste::item! {
//                async fn [< process_ $fn_suffix >](&self) -> Result<(), Self::Error>;
//                async fn [< stream_ $fn_suffix >](&self) -> Result<(), Self::Error>;
//                async fn [< stream_latest_ $fn_suffix >](&self) -> Result<(), Self::Error>;
//            }
//        }
//    };
//}

//define_processor_trait!(LogProcessor, logs, #[async_trait]);
//define_processor_trait!(BlockProcessor, blocks, #[async_trait]);

#[async_trait]
pub trait LogProcessor: Sync + Send {
    type Error;

    async fn process_logs(&self) -> Result<(), Self::Error>;
    async fn stream_logs(&self) -> Result<(), Self::Error>;
    async fn stream_latest_logs(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait BlockProcessor: Sync + Send {
    type Error;

    async fn process_blocks(&self) -> Result<(), Self::Error>;
    async fn stream_blocks(&self) -> Result<(), Self::Error>;
    async fn stream_latest_blocks(&self) -> Result<(), Self::Error>;
}
