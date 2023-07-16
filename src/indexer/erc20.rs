use ethereum_types::{H160, U256};

pub const TRANSFER_SIGNATURE: &[u8] = &[0xa9, 0x05, 0x9c, 0xbb];

#[derive(Debug)]
pub struct Transfer {
    pub from: H160,
    pub to: H160,
    pub value: U256,
}
