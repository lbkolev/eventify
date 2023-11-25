pub trait ContractFunction {}

#[macro_export]
macro_rules! contract_func {
    ($struct_name:ident [$($field_name:ident: $field_type:ty),* ]) => {
        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq)]
        pub struct $struct_name {
            #[serde(flatten)]
            boilerplate: $crate::transaction::TransactionBoilerplate,

            $(pub $field_name: $field_type),*
        }

        impl $crate::macros::ContractFunction for $struct_name {}

        impl $struct_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    boilerplate: $crate::transaction::TransactionBoilerplate::default(),
                    $($field_name),*
                }
            }

            $(
                pub fn $field_name(&mut self, $field_name: $field_type) -> &mut Self {
                    self.$field_name = $field_name;
                    self
                }
            )*

            /// Returns the contract address associated with the struct function call
            pub fn contract_addr(&self) -> ethers_core::types::H160 {
                self.boilerplate.contract_addr
            }

            pub fn transaction_hash(&self) -> ethers_core::types::H256 {
                self.boilerplate.transaction_hash
            }

            pub fn transaction_sender(&self) -> ethers_core::types::H160 {
                self.boilerplate.transaction_sender
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {:?}", stringify!($struct_name), self)
            }
        }
    };
}
