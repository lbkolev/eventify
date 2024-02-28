pub mod erc1155;
pub mod erc20;
pub mod erc4626;
pub mod erc721;
pub mod erc777;

use std::fmt::Debug;

use alloy_sol_types::sol;

sol! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    interface ERC20 {
        event Transfer(address from, address to, uint256 value);
        event Approval(address owner, address spender, uint256 value);
    }
}

sol! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    interface ERC721 {
        event Transfer(address from, address to, uint256 tokenId);
        event Approval(address owner, address approved, uint256 tokenId);
        event ApprovalForAll(address owner, address operator, bool approved);
    }
}

sol! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    interface ERC777 {
        event Sent(address operator, address from, address to, uint256 amount, bytes data, bytes operatorData);
        event Minted(address operator, address to, uint256 amount, bytes data, bytes operatorData);
        event Burned(address operator, address from, uint256 amount, bytes data, bytes operatorData);
        event AuthorizedOperator(address operator,address holder);
        event RevokedOperator(address operator, address holder);
    }
}

sol! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    interface ERC1155 {
        event TransferSingle(address operator, address from, address to, uint256 id, uint256 value);
        event TransferBatch(address operator, address from, address to, uint256[] ids, uint256[] values);
        event URI(string value, uint256 id);
    }
}

sol! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    interface ERC4626 {
        event Deposit(address sender, address owner, uint256 assets, uint256 shares);
        event Withdraw(address sender, address receiver, address owner, uint256 assets, uint256 shares);
    }
}
