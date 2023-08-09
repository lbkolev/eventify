use async_trait::async_trait;
use ethereum_types::{H160, H256, U256};

use crate::{contract_func, tx::IndexedTransaction, Insertable, Result};

pub const ERC721_APPROVE_SIGNATURE: &[u8] = &[0x09, 0x5e, 0xa7, 0xb3];

pub const ERC721_TRANSFER_FROM_SIGNATURE: &[u8] = &[0x23, 0xb8, 0x72, 0xdd];

pub const ERC721_SAFE_TRANSFER_FROM_SIGNATURE: &[u8] = &[0x42, 0x84, 0x2e, 0x0e];

pub const ERC721_SAFE_TRANSFER_FROM_WITH_DATA_SIGNATURE: &[u8] = &[0xb8, 0x8d, 0x4f, 0xde];
