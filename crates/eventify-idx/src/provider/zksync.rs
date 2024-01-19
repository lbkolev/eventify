//#[derive(Clone, Debug)]
//pub struct Zksync {
//    inner: Arc<WsClient>,
//}
//
//impl Zksync {
//    pub async fn new(host: String) -> Result<Self, NodeClientError> {
//        Ok(Self {
//            inner: Arc::new(
//                WsClientBuilder::default()
//                    .build(&host)
//                    .await
//                    .map_err(|_| NodeClientError::Connect)?,
//            ),
//        })
//    }
//
//    async fn get_block_number(&self) -> Result<BlockNumber, NodeClientError> {
//        let s: Result<String, NodeClientError> = self
//            .inner
//            .request("eth_blockNumber", rpc_params![])
//            .await
//            .map_err(|_| NodeClientError::LatestBlock);
//
//        if let Ok(s) = s {
//            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
//        } else {
//            Err(NodeClientError::LatestBlock)
//        }
//    }
//}
//
