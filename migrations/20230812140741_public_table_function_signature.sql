-- Add migration script here
CREATE TABLE public.function_signature (
    bytes_sig bytea NOT NULL,
    text_sig text NOT NULL,

    PRIMARY KEY(bytes_sig)
);

comment on TABLE public.function_signature is 'Each signature is a mapping between the human readable function signature and the 4-byte function selector used by the EVM.';
comment on COLUMN public.function_signature.bytes_sig is 'The 4-byte function selector used by the EVM.';
comment on COLUMN public.function_signature.text_sig is 'The human readable function signature.';
