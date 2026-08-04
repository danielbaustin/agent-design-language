# #5698 Design: Runtime v3 redb durable state

## Intent

Make Runtime v3 kernel-local checkpoint and lifelog persistence use one real
`redb` durable state authority instead of parallel flat files. The production
kernel must use the same configured absolute state root and writer ownership
boundary for checkpoints, lifelog entries, restart recovery, and corruption
handling.

## Boundaries

- Add a small Runtime v3 durable state module backed by `redb`.
- Replace production `checkpoint.json` and `lifelog.jsonl` writes in the local
  Runtime v3 adapters with atomic `redb` transactions.
- Preserve request principal binding, payload/state hashes, generation order,
  redaction flags, and fail-closed restore behavior.
- Keep #5344 as the stacked launch/soak dependency at exact head `ca242a5a`.

## Non-goals

- No Runtime v2 deletion.
- No remote database, service, ORM, wrapper, fixture, simulation, or degraded
  fallback.
- No scheduler redesign beyond preserving existing public adapter behavior.

## Validation

Run focused Runtime v3 adapter persistence tests, corruption and restart
negative tests, strict Clippy, and one exact pre-PR review before publication.
