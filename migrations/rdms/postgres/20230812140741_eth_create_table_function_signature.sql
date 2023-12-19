-- Add migration script here
CREATE TABLE eth.function_signature (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    hex_sig bytea NOT NULL,
    text_sig text UNIQUE NOT NULL,

    PRIMARY KEY(id)
);

comment on TABLE eth.function_signature is 'Each signature is a mapping between the human readable function signature and the 4-byte function selector used by the EVM.';
comment on COLUMN eth.function_signature.hex_sig is 'The hexadecimal representation of the 4-byte function selector used by the EVM.';
comment on COLUMN eth.function_signature.text_sig is 'The human readable function signature.';
