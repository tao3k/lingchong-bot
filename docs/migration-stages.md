# Migration Stages

This repository advances the Daochang migration through stage gates instead of
one-helper-at-a-time imports.

## M4-A Channel Contract Stabilization

Goal: make `lingchong-bot` the stable owner for bot-owned channel contracts that
do not require a live Telegram or Discord service.

Owned surfaces:

- Runtime environment resolution under `LINGCHONG_*`.
- Telegram and Discord session partition contracts.
- Channel message formatting and turn mapping to the external gateway.
- Deterministic Telegram and Discord harness helpers.
- Typed Discord ACL probe catalog, selector parsing, and validation.

Exit gate:

```shell
direnv exec . just test-channel-contract
direnv exec . just check
```

## M4-B Live-Gated Bot Probe Handoff

Goal: move live-gated Telegram and Discord probe launchers into this repository
without importing Daochang native tools or Wendao domain execution.

Required shape:

- All live probes are opt-in through explicit environment variables.
- All gateway calls use the external agent gateway client.
- No `XIUXIAN_DAOCHANG_*` default environment names are introduced.
- Any compatibility notes stay in docs, not in default commands.

Exit gate:

```shell
direnv exec . just stage-m4b
```

plus live-gated probe commands when their required tokens and local endpoints
are configured.

Current landed surface:

- `lingchong-bot preflight` validates the secret-safe runtime configuration and
  blocks live-gated execution when neither Telegram nor Discord is enabled.
- `just preflight` runs the same command through the repository toolchain.
- `scripts/channel/preflight.sh` is the shell handoff point for later live
  Telegram and Discord launchers.
- `lingchong-bot plan-live` renders the secret-safe launch plan for enabled
  Telegram and Discord channels.
- `scripts/channel/plan-live.sh` is the shell handoff point for inspecting the
  launch plan before a live run.
- `just stage-m4b` is the combined deterministic validation gate for M4-B. It
  runs channel contracts, preflight, launch-plan rendering, formatting, tests,
  and clippy with placeholder channel tokens unless real live secrets are
  provided by the caller.
- This is a launcher precondition, not a replacement for channel runtime
  execution yet.

Validation:

```shell
direnv exec . just stage-m4b
```

## M4-C Main-Repo Legacy Script Retirement

Goal: remove or quarantine the old main-repo `scripts/channel` surfaces that are
now owned here.

Required shape:

- Main-repo retained helpers are renamed away from channel ownership when they
  are really Wendao/process helpers.
- Bot-owned scripts point to this repository or are deleted after replacement.
- Historical references remain only in clearly historical or migration docs.

Exit gate:

The main repository passes its cargo metadata, focused retained-helper checks,
and active-reference scan.
