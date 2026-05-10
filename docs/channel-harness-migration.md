# Channel Harness Migration

`xiuxian-artisan-workshop/scripts/channel` is a mixed legacy directory. It
contains bot harnesses, Telegram/Discord launchers, memory-channel probes, and a
small set of Wendao/process helpers. The migration must not copy the whole
directory into this repository blindly.

## Target Ownership

`lingchong-bot` should import and adapt only bot/channel harnesses:

- Telegram and Discord runtime launchers.
- Telegram and Discord black-box probes.
- Bot acceptance and command-event scenarios.
- Bot session, deduplication, ACL, ingress, and mention-policy tests.
- Bot memory-channel tests that exercise channel behavior through the external
  agent gateway.

The import must rename active surfaces away from `xiuxian-daochang` and toward
`lingchong-bot`. Compatibility wrappers may exist only as temporary migration
notes, not as default commands.

## Main-Repo Retained Helpers

The main Xiuxian repository should retain only helpers that are still owned by
Wendao or shared local infrastructure:

- Valkey process launch, stop, and healthcheck helpers.
- Wendao gateway, frontend, sentinel, and document-extract healthchecks.
- Process-compose runtime helpers used by the main repository.
- OCR probe helpers that target Wendao analyzer/runtime packages.

Those helpers should eventually move out of `scripts/channel` in the main
repository because their ownership is no longer channel-specific.

## Current Inventory Snapshot

The source directory currently has roughly 673 top-level entries. A first-pass
name-based split found:

| Bucket | Count | Migration action |
| --- | ---: | --- |
| Bot runtime launchers | 5 | Import and rename. |
| Telegram harness | 18 | Import and rename. |
| Discord harness | 52 | Import and rename. |
| Agent black-box harness | 265 | Import selectively after gateway boundary review. |
| Bot memory harness | 180 | Import only channel-facing tests; keep kernel memory tests in Xiuxian. |
| Wendao/process helpers | 35 | Keep in Xiuxian, then move to a non-channel helper directory. |
| Uncategorized support modules | 118 | Review by dependency graph before importing. |

## Import Rules

1. Do not import native-tool execution code.
2. Do not import direct Wendao/Qianji domain calls.
3. Route runtime turns through the external gateway client.
4. Rename default environment variables to `LINGCHONG_*`.
5. Keep live tests gated by explicit environment variables.
6. Preserve deterministic unit tests before adding live-gated tests.

## Next Slice

Start with the smallest runnable harness set:

1. Import Telegram/Discord parser and message-shape unit tests that do not
   require a live Xiuxian binary.
2. Adapt them to the current `src/channels`, `src/harness`, and `src/turn`
   boundaries.
3. Add a `just test-channel-harness` command in this repository.
4. Remove the corresponding active script tests from the main repository after
   this repository passes `cargo test` and the new harness command.

## Imported Slice 1

The first typed Rust harness slice is landed under `src/harness/`:

- `telegram_profile`: extracts required Telegram group profiles from runtime
  log lines with typed chat id, thread id, session key, and chat type
  boundaries.
- `telegram_session_matrix`: builds Telegram matrix session keys, memory result
  scopes, and admin group chat selections with typed chat, user, and thread id
  boundaries.
- `discord_ingress`: normalizes Discord ingress stress configuration through a
  named request type and Discord id newtypes.
- `discord_acl`: validates command-reply and JSON summary observations against
  the expected recipient, session key set, and Discord session-scope set. It
  also owns the typed Discord ACL probe case catalog and suite/case filtering
  logic, including selector parsing and de-duplication for future CLI
  adapters.

The source behavior was selected from legacy script tests that did not require a
live Xiuxian binary. The implementation intentionally avoids a Python bulk
import so the target package keeps strong Rust API contracts and the default
Rust project harness can catch stringly or broad constructor surfaces.

The dedicated validation entrypoint is:

```shell
direnv exec . just test-channel-harness
```

The broader stage contract entrypoint is:

```shell
direnv exec . just test-channel-contract
```

## Imported Runtime Config Slice

Telegram webhook bind and port resolution now belongs to
`src/runtime/config.rs` under `LINGCHONG_*` environment variables:

- `LINGCHONG_TELEGRAM_WEBHOOK_BIND`
- `LINGCHONG_TELEGRAM_WEBHOOK_PORT`

Discord ingress bind, path, and local URL resolution also belong to
`src/runtime/config.rs`:

- `LINGCHONG_DISCORD_INGRESS_BIND`
- `LINGCHONG_DISCORD_INGRESS_PATH`
- `LINGCHONG_DISCORD_INGRESS_URL`

The target repository intentionally does not preserve `XIUXIAN_DAOCHANG_*` or
`XIUXIAN_WENDAO_*` aliases for this runtime surface.

Runtime config now uses named domain types for gateway URLs, bind addresses,
ports, paths, and local URLs so later channel migrations do not rebuild a
stringly typed DTO surface.

The `preflight` CLI command is the first M4-B live-gated launcher boundary. It
loads the same runtime config, prints a secret-safe report, and blocks when no
bot channel is enabled.

The `plan-live` CLI command is the second M4-B boundary. It renders the
secret-safe channel launch plan for enabled Telegram and Discord surfaces,
without starting a runtime process.

The `stage-m4b` Justfile entrypoint is the combined validation gate for this
stage. It runs deterministic channel contract tests, preflight, launch-plan
rendering, formatting, tests, and clippy. It uses placeholder channel tokens
when the caller has not supplied live secrets, so the gate remains deterministic
and safe for local development.

## Imported Session Contract Slice

Discord session scope construction now belongs to `src/channels/session.rs`.
This replaces the legacy ACL probe helper that assembled `json_session_scope`
values outside the bot runtime contract.

Harness tests are split under `tests/unit/harness/` by channel concern so the
test surface follows the same folder-first policy as production code.
