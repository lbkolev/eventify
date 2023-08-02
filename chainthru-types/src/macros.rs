#[macro_export]
macro_rules! contract_func {
    ($struct_name:ident [$($field_name:ident: $field_type:ty),* ]) => {
        #[derive(Builder, Clone, Debug, Default)]
        pub struct $struct_name {
            $(pub $field_name: $field_type),*
        }

        impl $struct_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {:?}", stringify!($struct_name), self)
            }
        }
    };
}

mod tests {
    #[test]
    fn test_contract_func() {
        contract_func!(TestContractFunc[field1: u32, field2: String]);

        let test_struct = TestContractFunc::new(1, "test".to_string());

        assert_eq!(test_struct.field1, 1);
        assert_eq!(test_struct.field2, "test");
    }
}
