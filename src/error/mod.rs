mod api;
mod body;
mod doc;

pub use api::AppError;
pub use doc::StandardErrorResponses;

pub type Result<T> = std::result::Result<axum::Json<T>, AppError>;
