#!/usr/bin/env bash
# M4-B live-gated handoff stage validation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${PROJECT_ROOT}"

cargo test --test unit_test channels:: -- --nocapture
cargo test --test unit_test session:: -- --nocapture
cargo test --test unit_test runtime:: -- --nocapture
cargo test --test unit_test turn:: -- --nocapture
cargo test --test unit_test harness:: -- --nocapture

export LINGCHONG_AGENT_GATEWAY_URL="${LINGCHONG_AGENT_GATEWAY_URL:-http://127.0.0.1:18093}"
export LINGCHONG_TELEGRAM_BOT_TOKEN="${LINGCHONG_TELEGRAM_BOT_TOKEN:-stage-m4b-telegram-token}"
export LINGCHONG_DISCORD_BOT_TOKEN="${LINGCHONG_DISCORD_BOT_TOKEN:-stage-m4b-discord-token}"

bash scripts/channel/preflight.sh
bash scripts/channel/plan-live.sh

cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
