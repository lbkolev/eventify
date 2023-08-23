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

mod tests {}
