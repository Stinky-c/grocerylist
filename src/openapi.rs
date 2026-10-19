use utoipa::openapi::security::{
    ApiKey, ApiKeyValue, Http, HttpAuthScheme, HttpBuilder, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some(r"Header authentication with a JWT following `Authorization: Bearer <token>`"))
                        .build(),
                ),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
            (name = "grocerylist", description = "Todo items management API")
    )
)]
pub struct ApiDoc;
