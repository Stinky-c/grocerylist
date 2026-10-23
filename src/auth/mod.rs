mod jwt;
mod session;

pub use jwt::{claim::Claim, extract::Jwt};
pub use session::Session;
use uuid::Uuid;


pub fn format_jti(id: &Uuid) -> String {
    format!("1:session/{}", id)
}
