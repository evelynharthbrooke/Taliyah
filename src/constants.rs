/// The user agent used for the reqwest client.
pub const REQWEST_USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
