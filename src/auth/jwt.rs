#[derive(serde::Deserialize)]
pub struct Claim {
    pub user_id: String,

    // https://www.rfc-editor.org/info/rfc7519/#section-4.1
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

const JWT_EXPIRY_SECS: u64 = 7 * 60 * 60 * 24 * 7;
impl Claim {
    pub fn new(user_id: String) -> Self {
        let now = jsonwebtoken::get_current_timestamp();
        Self {
            user_id,
            iss: env!("CARGO_CRATE_NAME").to_string(),
            iat: now,
            exp: (now + JWT_EXPIRY_SECS),
        }
    }
}
