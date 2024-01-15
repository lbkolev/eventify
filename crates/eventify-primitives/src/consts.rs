/*
 * event Transfer(address indexed _from, address indexed _to, uint256 _value)
 * event Approval(address indexed _owner, address indexed _spender, uint256 _value)
 */
/// https://eips.ethereum.org/EIPS/eip-20
pub const EVENTS_ETH_ERC20: [&str; 2] = [
    "Transfer(address,address,uint256)",
    "Approval(address,address,uint256)",
];

/*
 * event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId)
 * event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId)
 * event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved)
 */
/// https://eips.ethereum.org/EIPS/eip-721
pub const EVENTS_ETH_ERC721: [&str; 3] = [
    "Transfer(address,address,uint256)",
    "Approval(address,address,uint256)",
    "ApprovalForAll(address,address,bool)",
];

/*
 * event Sent(address indexed operator, address indexed from, address indexed to, uint256 amount, bytes data, bytes operatorData)
 * event Minted(address indexed operator, address indexed to, uint256 amount, bytes data, bytes operatorData)
 * event Burned(address indexed operator, address indexed from, uint256 amount, bytes data, bytes operatorData)
 * event AuthorizedOperator(address indexed operator,address indexed holder)
 * event RevokedOperator(address indexed operator, address indexed holder)
 */
/// https://eips.ethereum.org/EIPS/eip-777
pub const EVENTS_ETH_ERC777: [&str; 5] = [
    "Sent(address,address,address,uint256,bytes,bytes)",
    "Minted(address,address,uint256,bytes,bytes)",
    "Burned(address,address,uint256,bytes,bytes)",
    "AuthorizedOperator(address,address)",
    "RevokedOperator(address,address)",
];

/*
 * event TransferSingle(address indexed _operator, address indexed _from, address indexed _to, uint256 _id, uint256 _value)
 * event TransferBatch(address indexed _operator, address indexed _from, address indexed _to, uint256[] _ids, uint256[] _values)
 * event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved)
 * event URI(string _value, uint256 indexed _id)
 */
/// https://eips.ethereum.org/EIPS/eip-1155
pub const EVENTS_ETH_ERC1155: [&str; 4] = [
    "TransferSingle(address,address,address,uint256,uint256)",
    "TransferBatch(address,address,address,uint256[],uint256[])",
    "ApprovalForAll(address,address,bool)",
    "URI(string,uint256)",
];

/*
 * event Deposit(address indexed sender, address indexed owner, uint256 assets, uint256 shares)
 * event Withdraw(address indexed sender, address indexed receiver, address indexed owner, uint256 assets, uint256 shares)
 */
/// https://eips.ethereum.org/EIPS/eip-4626
pub const EVENTS_ETH_ERC4626: [&str; 2] = [
    "Deposit(address,address,uint256,uint256)",
    "Withdraw(address,address,address,uint256,uint256)",
];
