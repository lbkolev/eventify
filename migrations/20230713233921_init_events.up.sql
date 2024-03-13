/*
 * Tables
 */
--- function signatures
CREATE TABLE IF NOT EXISTS fn_sig (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    hex_sig BYTEA NOT NULL,
    text_sig TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(id)
);
comment on table fn_sig is 'Function signatures taken from https://4byte.directory, not associated with the collecting engine, but useful for debugging';
comment on column fn_sig.hex_sig is 'hexadecimal representation of the 4-byte function selector used by the EVM';
comment on column fn_sig.text_sig is 'human readable function signature';

--- ERC20 transfer events
CREATE TABLE IF NOT EXISTS erc20_transfer (
    tx_hash BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    "value" BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC20 approval events
CREATE TABLE IF NOT EXISTS erc20_approval (
    tx_hash BYTEA,
    "owner" BYTEA,
    spender BYTEA,
    "value" BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC721 transfer events
CREATE TABLE IF NOT EXISTS erc721_transfer (
    tx_hash BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    token_id BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC721 approval events
CREATE TABLE IF NOT EXISTS erc721_approval (
    tx_hash BYTEA,
    "owner" BYTEA,
    approved BYTEA,
    token_id BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC721/1155 approvalForAll events
CREATE TABLE IF NOT EXISTS erc_approval_for_all (
    tx_hash BYTEA,
    "owner" BYTEA,
    operator BYTEA,
    approved BOOLEAN,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC777 sent events
CREATE TABLE IF NOT EXISTS erc777_sent (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    amount BYTEA,
    "data" BYTEA,
    operator_data BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);
--- ERC777 minted events
CREATE TABLE IF NOT EXISTS erc777_minted (
    tx_hash BYTEA,
    operator BYTEA,
    "to" BYTEA,
    amount BYTEA,
    "data" BYTEA,
    operator_data BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC777 burned events
CREATE TABLE IF NOT EXISTS erc777_burned (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    amount BYTEA,
    "data" BYTEA,
    operator_data BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC777 authorizedOperator events
CREATE TABLE IF NOT EXISTS erc777_authorized_operator (
    tx_hash BYTEA,
    operator BYTEA,
    holder BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC777 revokedOperator events
CREATE TABLE IF NOT EXISTS erc777_revoked_operator (
    tx_hash BYTEA,
    operator BYTEA,
    holder BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC1155 transferSingle events
CREATE TABLE IF NOT EXISTS erc1155_transfer_single (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    id BYTEA,
    "value" BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC1155 transferBatch events
CREATE TABLE IF NOT EXISTS erc1155_transfer_batch (
    tx_hash BYTEA,
    operator BYTEA,
    "from" BYTEA,
    "to" BYTEA,
    ids BYTEA[],
    "values" BYTEA[],
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC1155 uri events
CREATE TABLE IF NOT EXISTS erc1155_uri (
    tx_hash BYTEA,
    "value" TEXT,
    id BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC4626 deposit events
CREATE TABLE IF NOT EXISTS erc4626_deposit (
    tx_hash BYTEA,
    sender BYTEA,
    "owner" BYTEA,
    "assets" BYTEA,
    shares BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);

--- ERC4626 withdraw events
CREATE TABLE IF NOT EXISTS erc4626_withdraw (
    tx_hash BYTEA,
    sender BYTEA,
    "receiver" BYTEA,
    "owner" BYTEA,
    "assets" BYTEA,
    shares BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(tx_hash)
);
---

/*
 * Indexes
 */
CREATE INDEX IF NOT EXISTS erc20_transfer_from_index ON erc20_transfer ("from");
CREATE INDEX IF NOT EXISTS erc20_transfer_to_index ON erc20_transfer ("to");

CREATE INDEX IF NOT EXISTS erc20_approval_owner_index ON erc20_approval (owner);
CREATE INDEX IF NOT EXISTS erc20_approval_spender_index ON erc20_approval (spender);

CREATE INDEX IF NOT EXISTS erc721_transfer_from_index ON erc721_transfer ("from");
CREATE INDEX IF NOT EXISTS erc721_transfer_to_index ON erc721_transfer ("to");

CREATE INDEX IF NOT EXISTS erc721_approval_owner_index ON erc721_approval (owner);

CREATE INDEX IF NOT EXISTS erc_approval_for_all_owner_index ON erc_approval_for_all (owner);
CREATE INDEX IF NOT EXISTS erc_approval_for_all_operator_index ON erc_approval_for_all (operator);

CREATE INDEX IF NOT EXISTS erc777_sent_from_index ON erc777_sent ("from");
CREATE INDEX IF NOT EXISTS erc777_sent_to_index ON erc777_sent ("to");
CREATE INDEX IF NOT EXISTS erc777_sent_operator_index ON erc777_sent (operator);

CREATE INDEX IF NOT EXISTS erc777_minted_to_index ON erc777_minted ("to");
CREATE INDEX IF NOT EXISTS erc777_minted_operator_index ON erc777_minted (operator);

CREATE INDEX IF NOT EXISTS erc777_burned_from_index ON erc777_burned ("from");
CREATE INDEX IF NOT EXISTS erc777_burned_operator_index ON erc777_burned (operator);

CREATE INDEX IF NOT EXISTS erc1155_transfer_single_from_index ON erc1155_transfer_single ("from");
CREATE INDEX IF NOT EXISTS erc1155_transfer_single_to_index ON erc1155_transfer_single ("to");
CREATE INDEX IF NOT EXISTS erc1155_transfer_single_operator_index ON erc1155_transfer_single (operator);
CREATE INDEX IF NOT EXISTS erc1155_transfer_single_id_index ON erc1155_transfer_single (id);

CREATE INDEX IF NOT EXISTS erc1155_transfer_batch_from_index ON erc1155_transfer_batch ("from");
CREATE INDEX IF NOT EXISTS erc1155_transfer_batch_to_index ON erc1155_transfer_batch ("to");
CREATE INDEX IF NOT EXISTS erc1155_transfer_batch_operator_index ON erc1155_transfer_batch (operator);

CREATE INDEX IF NOT EXISTS erc1155_uri_value_index ON erc1155_uri (value);
CREATE INDEX IF NOT EXISTS erc1155_uri_id_index ON erc1155_uri (id);

CREATE INDEX IF NOT EXISTS erc4626_deposit_tx_hash_index ON erc4626_deposit (tx_hash);
CREATE INDEX IF NOT EXISTS erc4626_deposit_sender_index ON erc4626_deposit (sender);
CREATE INDEX IF NOT EXISTS erc4626_deposit_owner_index ON erc4626_deposit (owner);

CREATE INDEX IF NOT EXISTS erc4626_withdraw_tx_hash_index ON erc4626_withdraw (tx_hash);
CREATE INDEX IF NOT EXISTS erc4626_withdraw_sender_index ON erc4626_withdraw (sender);
CREATE INDEX IF NOT EXISTS erc4626_withdraw_receiver_index ON erc4626_withdraw (receiver);
CREATE INDEX IF NOT EXISTS erc4626_withdraw_tx_owner_index ON erc4626_withdraw (owner);
---
