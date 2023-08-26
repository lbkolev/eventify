pub trait ContractFunction {}

#[macro_export]
macro_rules! contract_func {
    ($struct_name:ident [$($field_name:ident: $field_type:ty),* ]) => {
        #[derive(derive_builder::Builder, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq)]
        pub struct $struct_name {
            #[serde(flatten)]
            pub boilerplate: $crate::TransactionBoilerplate,

            $(pub $field_name: $field_type),*
        }

        impl $crate::macros::ContractFunction for $struct_name {}

        impl $struct_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    boilerplate: $crate::TransactionBoilerplate::default(),
                    $($field_name),*
                }
            }

            $(
                pub fn $field_name(&mut self, $field_name: $field_type) -> &mut Self {
                    self.$field_name = $field_name;
                    self
                }
            )*
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {:?}", stringify!($struct_name), self)
            }
        }
    };
}

mod tests {}
