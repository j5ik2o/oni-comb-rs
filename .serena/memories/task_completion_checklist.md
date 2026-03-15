# Task completion checklist
- If code is added back to the parser crate, make sure `parser/Cargo.toml` bench entries and actual bench files are consistent so Cargo can parse the manifest.
- Run formatting and tests once the crate builds again: `cargo fmt`, `cargo test -p oni-comb-parser`.
- If benchmarks are part of the task, ensure `cargo bench -p oni-comb-parser --no-run` succeeds before running full benches.
- When redesigning the parser core, compare against historical commits to verify which ideas were intentionally kept or discarded.
- Document compatibility breaks and migration notes because the reboot is allowed to be breaking.