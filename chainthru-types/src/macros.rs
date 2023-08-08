use ethereum_types::H256;

pub trait ContractFunction {}

#[macro_export]
macro_rules! contract_func {
    (contract=$contract_name:ident, $struct_name:ident [$($field_name:ident: $field_type:ty),* ]) => {
        #[derive(derive_builder::Builder, Clone, Debug, Default)]
        pub struct $struct_name {
            $(pub $field_name: $field_type),*
        }

        impl $crate::macros::ContractFunction for $struct_name {}

        impl $struct_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
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
/*
        #[$crate::async_trait]
        impl $crate::Insertable for $struct_name {
            async fn insert(&self, conn: &sqlx::PgPool) -> Result<(), sqlx::Error> {
                let sql = format!("
                    INSERT INTO {}.{} ({})
                    VALUES ({})
                    ON CONFLICT DO NOTHING
                    ",
                    $contract_name.to_lowercase(),
                    stringify!($struct_name).to_case($crate::Case::Snake).to_lowercase(),
                    stringify!($($field_name),*).to_lowercase(),
                    stringify!($($field_name),*).split(',').map(|_| "$").collect::<Vec<&str>>().join(", ")
                );

                sqlx::query(&sql)
                    $(if stringify!($field_type).starts_with("H") {
                        .bind(self.$field_name.as_bytes())
                    } else {
                        .bind(self.$field_name)
                    })*
                    .execute(conn)
                    .await?;

                sqlx::query(&sql)
                    $(.bind(self.$field_name.as_bytes()))*
                    .execute(conn)
                    .await?;

            }
        }
*/
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
