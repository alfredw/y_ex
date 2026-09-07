# Nerv Yex fork

This fork starts at upstream Yex `v0.10.5` and preserves the native changes
validated at `31c3118711d6c8e2636c6e3d0591f20e35a58ca9`:

- Versioned update/document inspection with complete-input decoding.
- State export retaining pending updates in both Yjs wire encodings.
- Yrs 0.25.0 pinned to `cd23b69ebbc42ba56febce32d9ef2731e0a3eae4`
  in `native/yex/Cargo.toml` and `Cargo.lock`.

## Mix packaging

Install Rust 1.93.0 (the checked-in rust-toolchain.toml selects it), then:

```sh
mix deps.get
mix compile
mix test test/nerv_packaging_test.exs
```

`Yex.Nif` uses Rustler 0.37.1 to build the native source in every environment.
Rustler 0.37.0 is retired for a non-workspace build defect. There is no upstream
precompiled-binary download or fallback. The resulting NIF lives under the Yex
application's `priv/native` directory and loads relative to that application.
Neither `NERV_SCHEMA_NATIVE` nor manual copying of BEAM files is required.
The dependency must be pinned by full Git revision when consumed by Nerv.

The existing fork test dependency Meck 0.9.2 needs
`ERL_COMPILER_OPTIONS='[nowarn_deprecated_catch]'` when compiling on OTP 29.
This affects fork tests only; consumers do not install Meck.

The packaging tests exercise the added inspection APIs, a relocated application,
and explicit failure when its native artifact is missing. Existing document,
text and sticky-index tests exercise the unchanged binding. Native implementation
and Cargo lockfile changes are outside this packaging change.

## Runtime boundary

Yex is a NIF binding: native operations execute inside the BEAM that loads it.
Nerv's document processing must load it only in application-owned child BEAMs.
Mix packaging does not establish crash containment or a production worker launcher.
The native patches are not a complete hostile-input audit.

The original `nerv/schema-inspection` reference commits remain reproducible with
their historical explicit loader. This packaging revision supersedes that loader;
it is not a drop-in replacement for scripts that compile the bindings manually.

## Maintenance

Keep upstream history and review changes explicitly. Update and verify Yrs first,
then advance Yex's manifest and Cargo lock pin, test the binding and Nerv's isolated
integration, and finally advance Nerv's Git pin. Never use baseline release NIFs
with the modified binding. Upstream: https://github.com/satoren/y_ex.
