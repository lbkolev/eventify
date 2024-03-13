use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use alloy_primitives::{Address, BlockNumber, B256};
use eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    #[serde(rename = "fromBlock", serialize_with = "serialize_block_number")]
    pub from: BlockNumber,
    #[serde(rename = "toBlock", serialize_with = "serialize_block_number")]
    pub to: BlockNumber,
    pub address: Option<Vec<Address>>,
    pub topics: Option<Vec<B256>>,
}

impl Criteria {
    pub fn new(
        from: BlockNumber,
        to: BlockNumber,
        address: Option<Vec<Address>>,
        topics: Option<Vec<B256>>,
    ) -> Criteria {
        Criteria {
            from,
            to,
            address,
            topics,
        }
    }
}

fn serialize_block_number<S>(x: &BlockNumber, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex = format!("0x{:x}", x);
    s.serialize_str(&hex)
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl FromStr for Criteria {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<&str> for Criteria {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("failed to parse criteria")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_criteria() {
        let json = serde_json::json!(
        {
            "fromBlock": 1,
            "toBlock": 2,
            "address": ["0x0000000000000000000000000000000000000001", "0x0000000000000000000000000000000000000002"],
            "topics": [
                "0x326edc3ac586176abeebb61e309fff4802231bb425463a8b205dc4c6bee35089",
                "0x24e6d6115f6446ccdee52f6150d0eb60d34dbe3072db714a11a601aeb3ee7503",
                "0x843f098f2784e8517a5ee9d050daa270e91403e1e4eaafa90bc535ef5910518c",
                "0x3ed3d15056b62bf1e6eae92a1eba5af40d8a7b42b2d05fdb94d1e3e1a61781c8"
            ]
        });
        let res = serde_json::from_value::<Criteria>(json).unwrap();
        assert_eq!(res.from, 1);
        assert_eq!(res.to, 2);
        assert_eq!(
            res.address,
            Some(vec![
                Address::from_str("0x0000000000000000000000000000000000000001").unwrap(),
                Address::from_str("0x0000000000000000000000000000000000000002").unwrap(),
            ]),
        );
        assert_eq!(
            res.topics,
            Some(vec![
                B256::from_str(
                    "0x326edc3ac586176abeebb61e309fff4802231bb425463a8b205dc4c6bee35089"
                )
                .unwrap(),
                B256::from_str(
                    "0x24e6d6115f6446ccdee52f6150d0eb60d34dbe3072db714a11a601aeb3ee7503"
                )
                .unwrap(),
                B256::from_str(
                    "0x843f098f2784e8517a5ee9d050daa270e91403e1e4eaafa90bc535ef5910518c"
                )
                .unwrap(),
                B256::from_str(
                    "0x3ed3d15056b62bf1e6eae92a1eba5af40d8a7b42b2d05fdb94d1e3e1a61781c8"
                )
                .unwrap()
            ]),
        );
    }

    #[test]
    fn test_deserialize_empty_criteria() {
        let json = serde_json::json!({});

        assert!(serde_json::from_value::<Criteria>(json).is_err());
    }
}
