-- indexes
DROP INDEX IF EXISTS block_hash_index;
DROP INDEX IF EXISTS block_number;

DROP INDEX IF EXISTS transaction_hash_index;
DROP INDEX IF EXISTS transaction_block_number_index;
DROP INDEX IF EXISTS transaction_from_index;
DROP INDEX IF EXISTS transaction_to_index;

DROP INDEX IF EXISTS log_address_index;
DROP INDEX IF EXISTS log_topic0_index;
DROP INDEX IF EXISTS log_tx_hash_index;
DROP INDEX IF EXISTS log_block_number_index;

DROP INDEX IF EXISTS contract_tx_hash_index;
DROP INDEX IF EXISTS contract_from_index;

DROP INDEX IF EXISTS transfer_from_index;
DROP INDEX IF EXISTS transfer_to_index;

DROP INDEX IF EXISTS approval_owner_index;
DROP INDEX IF EXISTS approval_spender_index;

DROP INDEX IF EXISTS approval_for_all_owner_index;
DROP INDEX IF EXISTS approval_for_all_operator_index;

DROP INDEX IF EXISTS sent_from_index;
DROP INDEX IF EXISTS sent_to_index;
DROP INDEX IF EXISTS sent_operator_index;

DROP INDEX IF EXISTS minted_to_index;
DROP INDEX IF EXISTS minted_operator_index;

DROP INDEX IF EXISTS burned_from_index;
DROP INDEX IF EXISTS burned_operator_index;

DROP INDEX IF EXISTS transfer_single_from_index;
DROP INDEX IF EXISTS transfer_single_to_index;
DROP INDEX IF EXISTS transfer_single_operator_index;
DROP INDEX IF EXISTS transfer_single_id_index;

DROP INDEX IF EXISTS transfer_batch_from_index;
DROP INDEX IF EXISTS transfer_batch_to_index;
DROP INDEX IF EXISTS transfer_batch_operator_index;

DROP INDEX IF EXISTS uri_value_index;
DROP INDEX IF EXISTS uri_id_index;

DROP INDEX IF EXISTS deposit_tx_hash_index;
DROP INDEX IF EXISTS deposit_sender_index;
DROP INDEX IF EXISTS deposit_owner_index;

DROP INDEX IF EXISTS withdraw_tx_hash_index;
DROP INDEX IF EXISTS withdraw_sender_index;
DROP INDEX IF EXISTS withdraw_receiver_index;
DROP INDEX IF EXISTS withdraw_tx_owner_index;

-- tables
DROP TABLE IF EXISTS eth.block;
DROP TABLE IF EXISTS eth.transaction;
DROP TABLE IF EXISTS eth.log;
DROP TABLE IF EXISTS eth.contract;
DROP TABLE IF EXISTS eth.fn_sig;
DROP TABLE IF EXISTS eth.transfer;
DROP TABLE IF EXISTS eth.approval;
DROP TABLE IF EXISTS eth.approval_for_all;
DROP TABLE IF EXISTS eth.sent;
DROP TABLE IF EXISTS eth.minted;
DROP TABLE IF EXISTS eth.burned;
DROP TABLE IF EXISTS eth.authorized_operator;
DROP TABLE IF EXISTS eth.revoked_operator;
DROP TABLE IF EXISTS eth.transfer_single;
DROP TABLE IF EXISTS eth.transfer_batch;
DROP TABLE IF EXISTS eth.uri;
DROP TABLE IF EXISTS eth.deposit;
DROP TABLE IF EXISTS eth.withdraw;

-- schema
DROP SCHEMA IF EXISTS eth;