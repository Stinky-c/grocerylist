use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    /// Stable, machine-readable error code.
    pub code: String,

    /// Human-readable message safe to expose to clients.
    pub message: String,

    /// Optional request correlation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
