//#[derive(Clone, Debug)]
//pub struct Zksync {
//    inner: Arc<WsClient>,
//}
//
//impl Zksync {
//    pub async fn new(host: String) -> Result<Self, NodeError> {
//        Ok(Self {
//            inner: Arc::new(
//                WsClientBuilder::default()
//                    .build(&host)
//                    .await
//                    .map_err(|_| NodeError::Connect)?,
//            ),
//        })
//    }
//
//    async fn get_block_number(&self) -> Result<BlockNumber, NodeError> {
//        let s: Result<String, NodeError> = self
//            .inner
//            .request("eth_blockNumber", rpc_params![])
//            .await
//            .map_err(|_| NodeError::LatestBlock);
//
//        if let Ok(s) = s {
//            Ok(BlockNumber::from_str_radix(s.trim_start_matches("0x"), 16)?)
//        } else {
//            Err(NodeError::LatestBlock)
//        }
//    }
//}
//
