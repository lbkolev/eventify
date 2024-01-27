/*
 * This migration initializes the ETH schema by creating all the required initial tables & indexes.
 */
CREATE SCHEMA IF NOT EXISTS eth;

--- raw block
CREATE TABLE IF NOT EXISTS eth.block (
    parent_hash BYTEA,
    uncles_hash BYTEA,
    coinbase BYTEA,
    root BYTEA,
    tx_hash BYTEA,
    receipt_hash BYTEA,
    difficulty BYTEA,
    number BIGINT,
    gas_limit BYTEA,
    gas_used BYTEA,
    time BIGINT,
    extra BYTEA,
    mix_digest BYTEA,
    nonce BYTEA,
    base_fee BIGINT,
    parent_beacon_root BYTEA,
    blob_gas_used BIGINT,
    excess_blob_gas BIGINT,
    withdraws_hash BYTEA,
    hash BYTEA UNIQUE,

    PRIMARY KEY(hash)
);
comment on table eth.block is 'Indexed blocks';
comment on column eth.block.hash is 'Hash of execution block';
comment on column eth.block.parent_hash is 'Hash of the parent block';
comment on column eth.block.uncles_hash is 'Uncle blocks are created when two or more miners create blocks at nearly the same time. Only one block can be mined and accepted as canonical on the blockchain. The others are uncle blocks, which are not included but still provide a reward to their miners for the work done.';
comment on column eth.block.root is 'root hash for the global state after applying changes in this block';
comment on column eth.block.tx_hash is 'root hash of the transactions in the payload';
comment on column eth.block.receipt_hash is 'hash of the transaction receipts trie';
comment on column eth.block.gas_limit is 'Maximum amount of gas that can be used by all transactions in this block';
comment on column eth.block.gas_used is 'Total amount of gas used by all transactions in this block';
comment on column eth.block.base_fee is 'The base fee value';

--- raw transaction
CREATE TABLE IF NOT EXISTS eth.transaction (
    block_hash BYTEA,
    block_number BIGINT,
    "from" BYTEA,
    gas BYTEA,
    gas_price BYTEA,
    hash BYTEA UNIQUE,
    input BYTEA,
    nonce BYTEA,
    "to" BYTEA,
    transaction_index INTEGER,
    value BYTEA,
    v BYTEA,
    r BYTEA,
    s BYTEA,

    PRIMARY KEY(hash)
);
comment on table eth.transaction is 'Indexed transactions';
comment on column eth.transaction.from is 'address that sent the transaction';
comment on column eth.transaction.to is 'address that received the transaction';

--- raw log
CREATE TABLE IF NOT EXISTS eth.log (
    id SERIAL,
    address BYTEA NOT NULL,
    topic0 BYTEA,
    topic1 BYTEA,
    topic2 BYTEA,
    topic3 BYTEA,
    data BYTEA NOT NULL,
    block_hash BYTEA,
    block_number BIGINT,
    tx_hash BYTEA,
    tx_index INTEGER,
    log_index INTEGER,
    removed BOOLEAN,

    UNIQUE (address, block_hash, tx_hash, data),
    PRIMARY KEY(id)
);
comment on table eth.log is 'Indexed logs that do not fit in any of the custom event tables e.g. Transfer & Approval from ERC20, Minted & Sent from ERC4626, etc';

--- raw contract details
CREATE TABLE IF NOT EXISTS eth.contract (
    tx_hash BYTEA,
    "from" BYTEA NOT NULL,
    input BYTEA NOT NULL,
    created_at TIMESTAMP with time zone NOT NULL DEFAULT now()
);

comment on table eth.contract is 'transactions creating contract';
comment on column eth.contract.tx_hash is 'hash of the transaction that created the contract';
comment on column eth.contract.from is 'address creator of the contract';
comment on column eth.contract.input is 'input data sent along with the transaction. Essentially the bytecode of the contract';

--- function signatures
CREATE TABLE IF NOT EXISTS eth.fn_sig (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    hex_sig BYTEA NOT NULL,
    text_sig TEXT UNIQUE NOT NULL,

    PRIMARY KEY(id)
);
comment on table eth.fn_sig is 'Function signatures taken from https://4byte.directory, not associated with the indexing engine, but useful for debugging';
comment on column eth.fn_sig.hex_sig is 'hexadecimal representation of the 4-byte function selector used by the EVM';
comment on column eth.fn_sig.text_sig is 'human readable function signature';

--- ERC20/721 transfer events
CREATE TABLE IF NOT EXISTS eth.transfer (
    tx_hash BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    "value" BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.transfer is 'Event used by ERC20, ERC721';
comment on column eth.transfer.tx_hash is 'Transaction hash the event got triggered by';

--- ERC20/721 approval events
CREATE TABLE IF NOT EXISTS eth.approval (
    tx_hash BYTEA,
    "owner" BYTEA,
    spender BYTEA,
    "value" BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.approval is 'Event used by ERC20, ERC721';
comment on column eth.approval.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.approval.owner is 'Owner of the resource';
comment on column eth.approval.spender is 'Spender of the resource';
comment on column eth.approval.value is 'Value|tokenId of the spent resource';

--- ERC721/1155 approvalForAll events
CREATE TABLE IF NOT EXISTS eth.approval_for_all (
    tx_hash BYTEA,
    "owner" BYTEA,
    operator BYTEA,
    approved BOOLEAN,

    PRIMARY KEY(tx_hash)
);
comment on table eth.approval_for_all is 'Event used by ERC721, 1155';
comment on column eth.approval_for_all.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.approval_for_all.owner is 'Owner of the resource';
comment on column eth.approval_for_all.operator is 'The operator granted or revoked access to the resource';
comment on column eth.approval_for_all.approved is 'Whether the resource is granted or revoked permissions';

--- ERC777 sent events
CREATE TABLE IF NOT EXISTS eth."sent" (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    amount BIGINT,
    "data" BYTEA,
    operator_data BYTEA,

    PRIMARY KEY(tx_hash)
);
comment on table eth.sent is 'Event used by ERC777 | Indicate a send of amount of tokens from the from address to the to address by the operator address';
comment on column eth.sent.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.sent.operator is 'Address which triggered the send';
comment on column eth.sent.from is 'Holder whose tokens were sent';
comment on column eth.sent.to is 'Recipient of the tokens';
comment on column eth.sent.amount is 'Number of tokens sent';
comment on column eth.sent.data is 'Information provided by the holder';
comment on column eth.sent.operator_data is 'Information provided by the operator';

--- ERC777 minted events
CREATE TABLE IF NOT EXISTS eth.minted (
    tx_hash BYTEA,
    operator BYTEA,
    "to" BYTEA,
    amount BIGINT,
    "data" BYTEA,
    operator_data BYTEA,

    PRIMARY KEY(tx_hash)
);
comment on table eth.minted is 'Event used by ERC777 | Indicate the minting of amount of tokens to the to address by the operator address';
comment on column eth.minted.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.minted.operator is 'Address which triggered the mint';
comment on column eth.minted.to is 'Recipient of the tokens';
comment on column eth.minted.amount is 'Number of tokens minted';
comment on column eth.minted.data is 'Information provided for the recipient';
comment on column eth.minted.operator_data is 'Information provided by the operator';

--- ERC777 burned events
CREATE TABLE IF NOT EXISTS eth.burned (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    amount BIGINT,
    "data" BYTEA,
    operator_data BYTEA,

    PRIMARY KEY(tx_hash)
);
comment on table eth.burned is 'Event used by ERC777 | Indicate the burning of amount of tokens from the from address by the operator address';
comment on column eth.burned.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.burned.operator is 'Address which triggered the burn';
comment on column eth.burned.from is 'Holder whose tokens were burned';
comment on column eth.burned.amount is 'Number of tokens burned';
comment on column eth.burned.data is 'Information provided by the holder';
comment on column eth.burned.operator_data is 'Information provided by the operator';

--- ERC777 authorizedOperator events
CREATE TABLE IF NOT EXISTS eth.authorized_operator (
    tx_hash BYTEA,
    operator BYTEA,
    holder BYTEA,

    PRIMARY KEY(tx_hash)
);
comment on table eth.authorized_operator is 'Event used by ERC777 | Indicates the authorization of operator as an operator for holder';
comment on column eth.authorized_operator.tx_hash is 'Transaction hash the event got triggered by';
comment on column eth.authorized_operator.operator is 'Address which became an operator of holder';
comment on column eth.authorized_operator.holder is 'Address of a holder which authorized the operator address as an operator';

--- ERC777 revokedOperator events
CREATE TABLE IF NOT EXISTS eth.revoked_operator (
    tx_hash BYTEA,
    operator BYTEA,
    holder BYTEA,

    PRIMARY KEY(tx_hash)
);
comment on table eth.revoked_operator is 'Event used by ERC777 | Indicates the revocation of operator as an operator for holder';
comment on column eth.revoked_operator.tx_hash is 'transaction hash the event got triggered by';
comment on column eth.revoked_operator.operator is 'address which was revoked as an operator of holder';
comment on column eth.revoked_operator.holder is 'address of a holder which revoked the operator address as an operator';

--- ERC1155 transferSingle events
CREATE TABLE IF NOT EXISTS eth.transfer_single (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    id BIGINT,
    "value" BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.transfer_single is 'Event used by ERC1155 | indicates a single balance transfer has occurred between a from and to pair';
comment on column eth.transfer_single.tx_hash is 'transaction hash the event got triggered by';
comment on column eth.transfer_single.operator is 'address of an account/contract that is approved to make the transfer';
comment on column eth.transfer_single.from is 'address of the holder whose balance is decreased';
comment on column eth.transfer_single.to is 'address of the recipient whose balance is increased';
comment on column eth.transfer_single.id is 'token type being transferred';
comment on column eth.transfer_single.value is 'the number of tokens the holder balance is decreased by and match what the recipient balance is increased by';

--- ERC1155 transferBatch events
CREATE TABLE IF NOT EXISTS eth.transfer_batch (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    ids BIGINT[],
    "values" BIGINT[],

    PRIMARY KEY(tx_hash)
);
comment on table eth.transfer_batch is 'Event used by ERC1155 | indicate multiple balance transfers have occurred between a from and to pair';
comment on column eth.transfer_batch.tx_hash is 'transaction hash the event got triggered by';
comment on column eth.transfer_batch.operator is 'address of an account/contract that is approved to make the transfer';
comment on column eth.transfer_batch.from is 'address of the holder whose balance is decreased for each entry pair in ids and values';
comment on column eth.transfer_batch.to is 'address of the recipient whose balance is increased for each entry pair in ids and values';
comment on column eth.transfer_batch.ids is 'contains the ids of the tokens being transferred';
comment on column eth.transfer_batch.values is 'contain the number of token to be transferred for each corresponding entry in ids';

--- ERC1155 uri events
CREATE TABLE IF NOT EXISTS eth.uri (
    tx_hash BYTEA,
    "value" TEXT,
    id BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.uri is 'Event used by ERC1155 | indicates the URI is updated for a token ID';
comment on column eth.uri.tx_hash is 'transaction hash the event got triggered by';
comment on column eth.uri.value is 'new URI';
comment on column eth.uri.id is 'token ID';

--- ERC4626 deposit events
CREATE TABLE IF NOT EXISTS eth.deposit (
    tx_hash BYTEA,
    sender BYTEA,
    "owner" BYTEA,
    "assets" BIGINT,
    shares BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.deposit is 'Event used by ERC4626 | sender has exchanged assets for shares, and transferred those shares to owner';
comment on column eth.deposit.tx_hash is 'transaction hash the event got triggered by';

--- ERC4626 withdraw events
CREATE TABLE IF NOT EXISTS eth.withdraw (
    tx_hash BYTEA,
    sender BYTEA,
    "receiver" BYTEA,
    "owner" BYTEA,
    "assets" BIGINT,
    shares BIGINT,

    PRIMARY KEY(tx_hash)
);
comment on table eth.withdraw is 'Event used by ERC4626 | sender has exchanged shares, owned by owner, for assets, and transferred those assets to receiver';
comment on column eth.withdraw.tx_hash is 'transaction hash the event got triggered by';

--- Indexes - built for tables & respectively columns used frequently in queries
CREATE INDEX IF NOT EXISTS block_hash_index ON eth.block (hash);
CREATE INDEX IF NOT EXISTS block_number ON eth.block (number);

CREATE INDEX IF NOT EXISTS transaction_hash_index ON eth.transaction (hash);
CREATE INDEX IF NOT EXISTS transaction_block_number_index ON eth.transaction (block_number);
CREATE INDEX IF NOT EXISTS transaction_from_index ON eth.transaction ("from");
CREATE INDEX IF NOT EXISTS transaction_to_index ON eth.transaction ("to");

CREATE INDEX IF NOT EXISTS log_address_index ON eth.log (address);
CREATE INDEX IF NOT EXISTS log_topic0_index ON eth.log (topic0);
CREATE INDEX IF NOT EXISTS log_tx_hash_index ON eth.log (tx_hash);
CREATE INDEX IF NOT EXISTS log_block_number_index ON eth.log (block_number);

CREATE INDEX IF NOT EXISTS contract_tx_hash_index ON eth.contract (tx_hash);
CREATE INDEX IF NOT EXISTS contract_from_index ON eth.contract ("from");

CREATE INDEX IF NOT EXISTS transfer_from_index ON eth.transfer ("from");
CREATE INDEX IF NOT EXISTS transfer_to_index ON eth.transfer ("to");

CREATE INDEX IF NOT EXISTS approval_owner_index ON eth.approval (owner);
CREATE INDEX IF NOT EXISTS approval_spender_index ON eth.approval (spender);

CREATE INDEX IF NOT EXISTS approval_for_all_owner_index ON eth.approval_for_all (owner);
CREATE INDEX IF NOT EXISTS approval_for_all_operator_index ON eth.approval_for_all (operator);

CREATE INDEX IF NOT EXISTS sent_from_index ON eth.sent ("from");
CREATE INDEX IF NOT EXISTS sent_to_index ON eth.sent ("to");
CREATE INDEX IF NOT EXISTS sent_operator_index ON eth.sent (operator);

CREATE INDEX IF NOT EXISTS minted_to_index ON eth.minted ("to");
CREATE INDEX IF NOT EXISTS minted_operator_index ON eth.minted (operator);

CREATE INDEX IF NOT EXISTS burned_from_index ON eth.burned ("from");
CREATE INDEX IF NOT EXISTS burned_operator_index ON eth.burned (operator);

CREATE INDEX IF NOT EXISTS transfer_single_from_index ON eth.transfer_single ("from");
CREATE INDEX IF NOT EXISTS transfer_single_to_index ON eth.transfer_single ("to");
CREATE INDEX IF NOT EXISTS transfer_single_operator_index ON eth.transfer_single (operator);
CREATE INDEX IF NOT EXISTS transfer_single_id_index ON eth.transfer_single (id);

CREATE INDEX IF NOT EXISTS transfer_batch_from_index ON eth.transfer_batch ("from");
CREATE INDEX IF NOT EXISTS transfer_batch_to_index ON eth.transfer_batch ("to");
CREATE INDEX IF NOT EXISTS transfer_batch_operator_index ON eth.transfer_batch (operator);

CREATE INDEX IF NOT EXISTS uri_value_index ON eth.uri (value);
CREATE INDEX IF NOT EXISTS uri_id_index ON eth.uri (id);

CREATE INDEX IF NOT EXISTS deposit_tx_hash_index ON eth.deposit(tx_hash);
CREATE INDEX IF NOT EXISTS deposit_sender_index ON eth.deposit(sender);
CREATE INDEX IF NOT EXISTS deposit_owner_index ON eth.deposit(owner);

CREATE INDEX IF NOT EXISTS withdraw_tx_hash_index ON eth.withdraw(tx_hash);
CREATE INDEX IF NOT EXISTS withdraw_sender_index ON eth.withdraw(sender);
CREATE INDEX IF NOT EXISTS withdraw_receiver_index ON eth.withdraw(receiver);
CREATE INDEX IF NOT EXISTS withdraw_tx_owner_index ON eth.withdraw(owner);
