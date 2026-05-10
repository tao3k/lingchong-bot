#!/usr/bin/env bash
# Secret-safe Lingchong bot live launch plan.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${PROJECT_ROOT}"

exec cargo run -- plan-live "$@"
