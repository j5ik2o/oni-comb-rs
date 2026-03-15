# oni-comb-rs project overview
- Purpose: reboot of a Rust parser combinator library (`oni-comb-parser`) after extracting the old public API into `API_SPEC.md` and removing the old implementation from the working tree.
- Current workspace layout: root Cargo workspace with member `parser`; actual implementation files are currently absent except for an empty `parser/src/lib.rs`.
- Historical context: git history contains the previous PoC parser implementation and later refactoring attempts (`Rc<dyn Fn>` based parser, `StaticParser` experiment, `ParseCursor` refactor). `API_SPEC.md` documents the old public API.
- Tech stack: Rust workspace; `criterion`, `nom`, `pom`, and `serde_json` were used in historical benches/dev dependencies.
- Current status: reboot skeleton; v2 design/implementation is expected to be built from scratch while reusing insights from history.
- Important files: `API_SPEC.md`, `parser/Cargo.toml`, git history under `parser/src/*` in older commits.