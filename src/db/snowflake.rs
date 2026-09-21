//! Snowflake, over its SQL REST API.
//!
//! There is no driver here because Snowflake publishes none for Rust, and the
//! community ones are tokio futures, which panic on GPUI's executor. What is
//! left is the documented HTTP API and a blocking client, which suits the rest
//! of this directory better than it sounds: every value arrives as a JSON
//! string, so hard rule 4's rendered text is most of the way there on arrival.
//!
//! The cost is that there is no session. A `USE` or an `ALTER SESSION` in one
//! submission does not reach the next, and neither does an open transaction.
//! That is the engine's documented behaviour and dbdelve does not paper over
//! it; inside one multi-statement submission they hold as usual.

/// What it takes to reach one database in one Snowflake account.
///
/// Its own struct rather than a `ServerConfig`: there is no port, no password
/// and no `sslmode` -- the API is HTTPS and always verified, so there is
/// nothing to weaken (hard rule 7, satisfied by having no field to set).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnowflakeConfig {
    /// The account identifier, which the token names regardless of which host
    /// the request is sent to.
    pub account: String,
    /// Blank derives the host from the account. Privatelink, a regional domain
    /// or a proxy is what fills it in.
    pub host: Option<String>,
    pub user: String,
    /// Path to an unencrypted PKCS#8 PEM private key. A path and not a secret,
    /// so it lives in the profile like `root_certificate` does and the Keychain
    /// holds nothing for this engine.
    pub private_key: String,
    /// One database per profile, as with Postgres. Every request carries it,
    /// which is why generated names stay two-part.
    pub database: String,
    /// Blank leaves the user's default in force, as does a blank role.
    pub warehouse: Option<String>,
    pub role: Option<String>,
    /// Seconds, or 0 for the account's own limit. Sent as a field of each
    /// request, never as SQL.
    pub statement_timeout: u32,
}

impl SnowflakeConfig {
    /// The host requests go to. The derived name is the service's documented
    /// default, overridable like any driver's default port.
    pub fn host(&self) -> String {
        match &self.host {
            Some(host) => host.clone(),
            None => format!("{}.snowflakecomputing.com", self.account),
        }
    }
}
