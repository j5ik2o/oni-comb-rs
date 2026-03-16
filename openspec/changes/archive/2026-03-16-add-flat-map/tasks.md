## 1. FlatMap combinator type

- [x] 1.1 Create `modules/parser/src/combinator/flat_map.rs` with `FlatMap<P, F>` struct (private fields)
- [x] 1.2 Implement `Parser<I>` for `FlatMap<P, F>` with transparent Fail propagation
- [x] 1.3 Register `flat_map` module in `modules/parser/src/combinator/mod.rs`

## 2. ParserExt integration

- [x] 2.1 Add `.flat_map(f)` method to `ParserExt` trait in `modules/parser/src/parser_ext.rs`
- [x] 2.2 Add `flat_map` constructor function (if needed) and re-export from prelude

## 3. Box<dyn Parser> support

- [x] 3.1 Implement `Parser<I>` for `Box<dyn Parser<I>>` to enable type-erased dynamic dispatch
- [x] 3.2 Verify that `flat_map` with `Box<dyn Parser>` compiles and works for heterogeneous branches

## 4. Tests

- [x] 4.1 Test: flat_map succeeds when both parsers succeed
- [x] 4.2 Test: flat_map fails when the first parser fails (Backtrack)
- [x] 4.3 Test: flat_map propagates Cut from the first parser
- [x] 4.4 Test: flat_map propagates failure from the dynamically chosen parser
- [x] 4.5 Test: flat_map with same-type branches (no Box needed)
- [x] 4.6 Test: flat_map with Box<dyn Parser> for heterogeneous branches
- [x] 4.7 Test: flat_map inside attempt downgrades Cut to Backtrack
- [x] 4.8 Test: flat_map result can be mapped
- [x] 4.9 Test: flat_map can be used inside or

## 5. Documentation

- [x] 5.1 Add flat_map to ParserExt method table in README.md
- [x] 5.2 Add usage examples (same-type and Box<dyn Parser>) to README.md
