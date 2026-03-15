# Style and conventions
- User-facing communication must be in Japanese.
- Naming rule from AGENTS.md: avoid ambiguous suffixes like `Manager`, `Util`, `Facade`, `Service`, `Runtime`, `Engine`; prefer concrete responsibility names such as `Registry`, `Policy`, `Coordinator`, `Factory`, `Adapter`, `Executor`, etc.
- Before adding code, study 2-3 similar existing implementations or historical implementations in the repo and follow project-specific patterns rather than generic textbook structure.
- Prefer simple, maintainable design; avoid over-abstraction and YAGNI violations.
- For Rust/public API design, keep one public type per file when adding real implementation files.
- For edits, use ASCII by default and keep comments sparse and high-signal.