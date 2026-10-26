use serde::Serialize;

/// Build metadata exposed at `/_status/version`.
///
/// The values come from the *application's* `build.rs` (git describe,
/// cargo profile, timestamp) — `env!` has to run in the crate whose
/// build script set the vars, so construct this there:
///
/// ```rust,ignore
/// pub const VERSION: Version = Version::new(
///     env!("REPO_VERSION"),
///     env!("BUILD_PROFILE"),
///     env!("BUILD_TIMESTAMP"),
/// );
/// ```
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Version {
    pub version: &'static str,
    pub build_profile: &'static str,
    pub build_timestamp: &'static str,
}

impl Version {
    pub const fn new(
        version: &'static str,
        build_profile: &'static str,
        build_timestamp: &'static str,
    ) -> Self {
        Self {
            version,
            build_profile,
            build_timestamp,
        }
    }
}
