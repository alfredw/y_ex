# Nerv reference fork

Branch `nerv/schema-inspection` starts at upstream `v0.10.5`
(`b58458099f0f243bf1890f88ae802c9866b3bc75`). It preserves the Yex patch from
Nerv's `reference/gh-11-durable-ledger` native-schema fixture:

- Versioned update/document inspection NIFs, with complete-input decoding.
- State export that retains pending updates in both Yjs wire encodings.
- The explicit `NERV_SCHEMA_NATIVE` loader used by the isolated reference worker.

Yrs is pinned to `alfredw/y-crdt` at the full revision in
`native/yex/Cargo.toml` and `Cargo.lock`. No sibling checkout or patch application
is needed. Build with Rust 1.93 and the locked Cargo dependencies:

```sh
cargo build --release --locked --manifest-path native/yex/Cargo.toml
```

This branch retains the reference loader intentionally. It requires a built NIF
and `NERV_SCHEMA_NATIVE` set to its absolute path without the library extension;
it does not download upstream precompiled binaries. Nerv's native-schema build
script compiles the Elixir bindings and installs the NIF for its child worker.
This is not yet a normal Mix dependency or production packaging decision.

Integration, malformed-input, recovery, and anchor tests live in
https://github.com/alfredw/kaizen-studio/tree/reference/gh-11-durable-ledger/scripts/native_schema.
Nerv retains child-process isolation; these patches are not a full hostile-input audit.

Upstream: https://github.com/satoren/y_ex. Keep `upstream` configured locally.
Review upstream changes on this branch, rerun integration tests, and update
downstream full commit pins only after verification. Never fetch baseline release
NIFs for this modified binding.
