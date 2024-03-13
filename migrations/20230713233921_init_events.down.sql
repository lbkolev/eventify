-- indexes
DROP INDEX IF EXISTS erc20_transfer_from_index;
DROP INDEX IF EXISTS erc20_transfer_to_index;

DROP INDEX IF EXISTS erc20_approval_owner_index;
DROP INDEX IF EXISTS erc20_approval_spender_index;

DROP INDEX IF EXISTS erc_approval_for_all_owner_index;
DROP INDEX IF EXISTS erc_approval_for_all_operator_index;

DROP INDEX IF EXISTS erc777_sent_from_index;
DROP INDEX IF EXISTS erc777_sent_to_index;
DROP INDEX IF EXISTS erc777_sent_operator_index;

DROP INDEX IF EXISTS erc777_minted_to_index;
DROP INDEX IF EXISTS erc777_minted_operator_index;

DROP INDEX IF EXISTS erc777_burned_from_index;
DROP INDEX IF EXISTS erc777_burned_operator_index;

DROP INDEX IF EXISTS erc1155_transfer_single_from_index;
DROP INDEX IF EXISTS erc1155_transfer_single_to_index;
DROP INDEX IF EXISTS erc1155_transfer_single_operator_index;
DROP INDEX IF EXISTS erc1155_transfer_single_id_index;

DROP INDEX IF EXISTS erc1155_transfer_batch_from_index;
DROP INDEX IF EXISTS erc1155_transfer_batch_to_index;
DROP INDEX IF EXISTS erc1155_transfer_batch_operator_index;

DROP INDEX IF EXISTS erc1155_uri_value_index;
DROP INDEX IF EXISTS erc1155_uri_id_index;

DROP INDEX IF EXISTS erc4626_deposit_tx_hash_index;
DROP INDEX IF EXISTS erc4626_deposit_sender_index;
DROP INDEX IF EXISTS erc4626_deposit_owner_index;

DROP INDEX IF EXISTS erc4626_withdraw_tx_hash_index;
DROP INDEX IF EXISTS erc4626_withdraw_sender_index;
DROP INDEX IF EXISTS erc4626_withdraw_receiver_index;
DROP INDEX IF EXISTS erc4626_withdraw_tx_owner_index;

-- tables
DROP TABLE IF EXISTS erc20_transfer;
DROP TABLE IF EXISTS erc20_approval;
DROP TABLE IF EXISTS erc721_transfer;
DROP TABLE IF EXISTS erc721_approval;
DROP TABLE IF EXISTS erc_approval_for_all;
DROP TABLE IF EXISTS erc777_sent;
DROP TABLE IF EXISTS erc777_minted;
DROP TABLE IF EXISTS erc777_burned;
DROP TABLE IF EXISTS erc777_authorized_operator;
DROP TABLE IF EXISTS erc777_revoked_operator;
DROP TABLE IF EXISTS erc1155_transfer_single;
DROP TABLE IF EXISTS erc1155_transfer_batch;
DROP TABLE IF EXISTS erc1155_uri;
DROP TABLE IF EXISTS erc4626_deposit;
DROP TABLE IF EXISTS erc4626_withdraw;
DROP TABLE IF EXISTS fn_sig;