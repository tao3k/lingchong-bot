# Daochang Migration

`lingchong-bot` is the new home for Telegram and Discord bot runtime surfaces.

The migration is staged:

1. Establish an independent Rust package and channel substrate.
2. Move bot/channel runtime behavior.
3. Keep native tools out of this repository.
4. Remove `xiuxian-daochang` from the main Xiuxian workspace only after this
   repository builds and tests independently.

## Native Tool Boundary

The old Daochang crate included native tools for Wendao search, Zhixing, spider
execution, and skill aliases. Those are domain execution surfaces rather than bot
surfaces. They must not be copied into `lingchong-bot`.

When the bot needs knowledge or memory behavior, it should call a stable gateway
or client endpoint owned by the Xiuxian/Wendao side. The first supported client
boundary is the external agent gateway `/message` route, with channel turns
mapped through `ChannelMessage` and `BotTurnService`.

## Channel Harness Import

See [Channel Harness Migration](channel-harness-migration.md) for the
script-level import plan. The legacy source directory mixes bot harnesses with
Wendao/process helpers, so the import must be selective rather than a bulk copy.
