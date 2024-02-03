-- indexes
DROP INDEX IF EXISTS block_hash_index;
DROP INDEX IF EXISTS block_number;

DROP INDEX IF EXISTS transaction_hash_index;
DROP INDEX IF EXISTS transaction_block_number_index;
DROP INDEX IF EXISTS transaction_from_index;
DROP INDEX IF EXISTS transaction_to_index;

DROP INDEX IF EXISTS contract_tx_hash_index;
DROP INDEX IF EXISTS contract_from_index;

DROP INDEX IF EXISTS log_address_index;
DROP INDEX IF EXISTS log_topic0_index;
DROP INDEX IF EXISTS log_block_number_index;

DROP INDEX IF EXISTS log_transfer_from_index;
DROP INDEX IF EXISTS log_transfer_to_index;

DROP INDEX IF EXISTS log_approval_owner_index;
DROP INDEX IF EXISTS log_approval_spender_index;

DROP INDEX IF EXISTS log_approval_for_all_owner_index;
DROP INDEX IF EXISTS log_approval_for_all_operator_index;

DROP INDEX IF EXISTS log_sent_from_index;
DROP INDEX IF EXISTS log_sent_to_index;
DROP INDEX IF EXISTS log_sent_operator_index;

DROP INDEX IF EXISTS log_minted_to_index;
DROP INDEX IF EXISTS log_minted_operator_index;

DROP INDEX IF EXISTS log_burned_from_index;
DROP INDEX IF EXISTS log_burned_operator_index;

DROP INDEX IF EXISTS log_transfer_single_from_index;
DROP INDEX IF EXISTS log_transfer_single_to_index;
DROP INDEX IF EXISTS log_transfer_single_operator_index;
DROP INDEX IF EXISTS log_transfer_single_id_index;

DROP INDEX IF EXISTS log_transfer_batch_from_index;
DROP INDEX IF EXISTS log_transfer_batch_to_index;
DROP INDEX IF EXISTS log_transfer_batch_operator_index;

DROP INDEX IF EXISTS log_uri_value_index;
DROP INDEX IF EXISTS log_uri_id_index;

DROP INDEX IF EXISTS log_deposit_tx_hash_index;
DROP INDEX IF EXISTS log_deposit_sender_index;
DROP INDEX IF EXISTS log_deposit_owner_index;

DROP INDEX IF EXISTS log_withdraw_tx_hash_index;
DROP INDEX IF EXISTS log_withdraw_sender_index;
DROP INDEX IF EXISTS log_withdraw_receiver_index;
DROP INDEX IF EXISTS log_withdraw_tx_owner_index;

-- tables
DROP TABLE IF EXISTS eth.block;
DROP TABLE IF EXISTS eth.transaction;
DROP TABLE IF EXISTS eth.contract;
DROP TABLE IF EXISTS eth.fn_sig;
DROP TABLE IF EXISTS eth.log;
DROP TABLE IF EXISTS eth.log_transfer;
DROP TABLE IF EXISTS eth.log_approval;
DROP TABLE IF EXISTS eth.log_approval_for_all;
DROP TABLE IF EXISTS eth.log_sent;
DROP TABLE IF EXISTS eth.log_minted;
DROP TABLE IF EXISTS eth.log_burned;
DROP TABLE IF EXISTS eth.log_authorized_operator;
DROP TABLE IF EXISTS eth.log_revoked_operator;
DROP TABLE IF EXISTS eth.log_transfer_single;
DROP TABLE IF EXISTS eth.log_transfer_batch;
DROP TABLE IF EXISTS eth.log_uri;
DROP TABLE IF EXISTS eth.log_deposit;
DROP TABLE IF EXISTS eth.log_withdraw;

-- schema
DROP SCHEMA IF EXISTS eth;