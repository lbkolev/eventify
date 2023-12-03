use serde_derive::{Deserialize, Serialize};

use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub(crate) struct CountResponse {
    /// The total number of whatever item we're querying for (Block|Tx|Log)
    #[schema(example = 133312)]
    pub(crate) count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub(crate) struct ErrorResponse {
    pub(crate) error: String,
}
