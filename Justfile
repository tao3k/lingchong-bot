set dotenv-load := true

fmt:
    cargo fmt --check

test:
    cargo test

clippy:
    cargo clippy --all-targets -- -D warnings

test-channel-harness:
    cargo test --test unit_test harness:: -- --nocapture

test-channel-contract:
    cargo test --test unit_test channels:: -- --nocapture
    cargo test --test unit_test session:: -- --nocapture
    cargo test --test unit_test runtime:: -- --nocapture
    cargo test --test unit_test turn:: -- --nocapture
    cargo test --test unit_test harness:: -- --nocapture

preflight:
    cargo run -- preflight

preflight-script:
    bash scripts/channel/preflight.sh

plan-live:
    cargo run -- plan-live

plan-live-script:
    bash scripts/channel/plan-live.sh

stage-m4b:
    bash scripts/channel/stage-m4b.sh

check:
    cargo fmt --check
    cargo test
    cargo clippy --all-targets -- -D warnings
