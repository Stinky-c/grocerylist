#![allow(dead_code)]

use crate::error::body::ErrorBody;
use utoipa::IntoResponses;

#[derive(IntoResponses)]
pub enum StandardErrorResponses {
    #[response(
        status = 400,
        description = "Invalid request",
        content_type = "application/json"
    )]
    BadRequest(#[to_schema] ErrorBody),
    /*
       #[response(
           status = 404,
           description = "Resource not found",
           content_type = "application/json"
       )]
       NotFound(#[to_schema] ErrorBody),

    */
    #[response(
        status = 500,
        description = "Internal server error",
        content_type = "application/json"
    )]
    InternalServerError(#[to_schema] ErrorBody),

    #[response(
        status = 400,
        description = "Missing authorization bearer",
        content_type = "application/json"
    )]
    MissingToken(#[to_schema] ErrorBody),
}
