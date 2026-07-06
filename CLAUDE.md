# recto — agent guide

Pure-Rust document SDK: high-fidelity DOCX read/write/edit, with conversion to PDF, HTML, Markdown, and PNG. Cargo workspace under `crates/`.

## Workspace docs (if present)

If a `specs/` directory exists at the repo root (a workspace-local mount, not part of this repository), **read `specs/README.md` first and follow its workflow and task board before doing anything else**. Documentation changes are committed in that directory's own repository, not here.

## Build & verify

```sh
cargo build --workspace
cargo test --workspace          # must stay green after every change
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Hard rules

- No GPL / AGPL dependencies. No shelling out to external converters (LibreOffice / Word).
- Deterministic output: same input + same build + same options ⇒ byte-identical output (PDF / PNG).
- Never break DOCX round-trip fidelity: unknown XML elements and attributes are preserved (never dropped); `rsid` / `paraId` and namespace prefixes survive read → write.
- No network access at runtime by default; no reading system fonts by default.
- The `recto-trace` JSON schema is a cross-repo contract; breaking changes require coordinated updates (see its rustdoc).
