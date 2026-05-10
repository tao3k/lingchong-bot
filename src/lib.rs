//! Telegram and Discord bot runtime substrate for Lingchong.
//!
//! This crate intentionally excludes Daochang native tools. It owns channel
//! behavior and bot-facing runtime surfaces only.

/// Channel-safe bot formatting and adapter substrate.
pub mod channels;
/// Command-line adapter for runtime diagnostics.
pub mod cli;
/// Client boundary for external agent gateway calls.
pub mod gateway;
/// Deterministic channel harness helpers migrated from legacy scripts.
pub mod harness;
/// Environment-backed runtime configuration.
pub mod runtime;
/// Channel turn orchestration against the external gateway.
pub mod turn;

#[cfg(test)]
rust_lang_project_harness::rust_project_harness_cargo_test_gate!(
    config = {
        rust_lang_project_harness::default_rust_harness_config().with_verification_profile_hint(
            rust_lang_project_harness::RustVerificationProfileHint::new(
                "src/lib.rs",
                [rust_lang_project_harness::RustOwnerResponsibility::PublicApi],
            )
            .with_rationale("crate root owns the public package API for cargo-test verification"),
        )
    }
);
