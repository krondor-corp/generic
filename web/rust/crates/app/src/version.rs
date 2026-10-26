use generic_rust_core::Version;

/// Build metadata baked in by `build.rs`; served at `/_status/version`.
pub const VERSION: Version = Version::new(
    env!("REPO_VERSION"),
    env!("BUILD_PROFILE"),
    env!("BUILD_TIMESTAMP"),
);
