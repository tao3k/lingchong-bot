# lingchong-bot

`lingchong-bot` owns the Telegram and Discord bot runtime surface that is being
split out of `xiuxian-artisan-workshop`.

## Ownership

This repository owns:

- Telegram and Discord bot adapters.
- Channel-safe message formatting and chunking.
- Gateway/client calls to an external agent service.
- Channel turn mapping from inbound bot messages to gateway `/message` calls.
- Bot runtime configuration and deployment documentation.
- Bot-specific mock and live-gated tests.
- Deterministic channel harness helpers for parser/config validation.

This repository does not own:

- Wendao search, indexing, or knowledge-domain algorithms.
- Qianji workflow execution.
- Zhixing tools.
- Daochang native-tool registries or skill aliases.

Bot-to-Wendao integration must use explicit gateway or client boundaries. Native
tools stay in the Xiuxian kernel/domain side or move to a separate service
contract later.

## Development

```shell
direnv exec . cargo fmt --check
direnv exec . cargo test
direnv exec . cargo clippy --all-targets -- -D warnings
direnv exec . just test-channel-harness
direnv exec . just test-channel-contract
direnv exec . just stage-m4b
```

`cargo test` includes the repository's self-applied
`rust-lang-project-harness` policy gate. The harness dependency is pinned in
`Cargo.toml` because this repository is expected to follow the same Rust project
maintenance contract as the Xiuxian crates it was split from.

## Channel Harness

The first migrated harness slice is implemented as typed Rust helpers under
`src/harness/`, not as a bulk import of legacy Python scripts. It currently
covers:

- Telegram group-profile extraction from runtime logs.
- Telegram session-matrix key/scope planning and admin group selection.
- Discord ingress stress configuration normalization.
- Discord ACL command-reply and JSON session-scope validation.

This keeps the target repo aligned with its channel ownership while avoiding
old in-process Daochang/native-tool coupling.

The dedicated harness entrypoint is:

```shell
direnv exec . just test-channel-harness
```

The stage-level channel contract entrypoint is:

```shell
direnv exec . just test-channel-contract
```

## Runtime Configuration

The first executable surface is a configuration check:

```shell
LINGCHONG_AGENT_GATEWAY_URL=http://127.0.0.1:18093 \
  direnv exec . cargo run -- check-config
```

Live-gated launchers should call preflight before starting channel processes:

```shell
LINGCHONG_AGENT_GATEWAY_URL=http://127.0.0.1:18093 \
LINGCHONG_TELEGRAM_BOT_TOKEN=... \
  direnv exec . cargo run -- preflight
```

The shell entrypoint for staged launcher handoff is:

```shell
LINGCHONG_AGENT_GATEWAY_URL=http://127.0.0.1:18093 \
LINGCHONG_TELEGRAM_BOT_TOKEN=... \
  direnv exec . just preflight-script
```

The secret-safe launch-plan entrypoint is:

```shell
LINGCHONG_AGENT_GATEWAY_URL=http://127.0.0.1:18093 \
LINGCHONG_TELEGRAM_BOT_TOKEN=... \
  direnv exec . just plan-live-script
```

The M4-B stage gate runs the deterministic channel contracts, secret-safe
preflight, secret-safe launch-plan rendering, formatting, tests, and clippy:

```shell
direnv exec . just stage-m4b
```

Supported environment variables:

- `LINGCHONG_AGENT_GATEWAY_URL`: required external agent gateway base URL.
- `LINGCHONG_TELEGRAM_BOT_TOKEN`: enables Telegram runtime when present.
- `LINGCHONG_TELEGRAM_WEBHOOK_SECRET`: marks Telegram webhook protection as
  configured when present.
- `LINGCHONG_TELEGRAM_WEBHOOK_BIND`: optional local Telegram webhook bind
  address.
- `LINGCHONG_TELEGRAM_WEBHOOK_PORT`: optional local Telegram webhook port.
  Defaults to the port embedded in `LINGCHONG_TELEGRAM_WEBHOOK_BIND`, then
  `18081`.
- `LINGCHONG_TELEGRAM_API_BASE_URL`: optional Telegram API base URL, defaulting
  to `https://api.telegram.org`.
- `LINGCHONG_TELEGRAM_SESSION_PARTITION`: planned Telegram session policy,
  using `chat`, `chat_user`, `user`, or `chat_thread_user`.
- `LINGCHONG_DISCORD_BOT_TOKEN`: enables Discord runtime when present.
- `LINGCHONG_DISCORD_INGRESS_SECRET`: marks Discord ingress protection as
  configured when present.
- `LINGCHONG_DISCORD_INGRESS_BIND`: optional local Discord ingress bind
  address, defaulting to `127.0.0.1:18082`.
- `LINGCHONG_DISCORD_INGRESS_PATH`: optional local Discord ingress path,
  defaulting to `/discord/ingress`.
- `LINGCHONG_DISCORD_INGRESS_URL`: optional explicit local Discord ingress URL
  for probes. Defaults to the normalized bind and path.
- `LINGCHONG_DISCORD_REQUIRE_MENTION`: optional boolean, defaulting to `true`.
- `LINGCHONG_DISCORD_SESSION_PARTITION`: planned Discord session policy, using
  `guild_channel_user`, `channel`, `user`, or `guild_user`.

The `check-config` output is secret-safe and reports only enablement and
presence flags.

## Migration Notes

The first migration slice intentionally starts with the channel substrate instead
of copying the whole `xiuxian-daochang` crate. This prevents Wendao/Qianji/native
tool ownership from being recreated inside the bot repository.
