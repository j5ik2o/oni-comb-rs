# Suggested commands
- `mise ls` : show tool versions configured for this repo.
- `sed -n '1,220p' API_SPEC.md` : inspect the extracted old public API.
- `git log --oneline -- modules/parser` : inspect parser crate history.
- `git show <commit>:parser/src/core/parser.rs | sed -n '1,260p'` : inspect historical parser core implementation.
- `cargo test -p oni-comb-parser` : intended test command, but currently fails because `modules/parser/Cargo.toml` references missing bench files.
- `cargo bench -p oni-comb-parser --no-run` : intended benchmark check, currently fails for the same manifest reason.
- `rg --files modules/parser` / `find modules/parser -maxdepth 3 -type f` : inspect current crate contents.
- `git grep -n -E 'Rc<|Rc::new|dyn Fn|\.clone\(' <commit> -- modules/parser/src` : inspect historical overhead hotspots.