## [3.0.0](https://github.com/j5ik2o/oni-comb-rs/compare/oni-comb-parser-v2.2.0...oni-comb-parser-v3.0.0) (2026-03-18)


### ⚠ BREAKING CHANGES

* **parser:** `Input` trait now requires `type Error: ExpectError`.
`ParseError` factory methods (`expected_char`, `expected_tag`,
`expected_description`, `expected_eof`) are removed; use
`ExpectError::from_expected(pos, Expected::Variant(...))` instead.

- Add `ExpectError` trait and `MinimalError` type for core-only error handling
- Add `Input::Error` associated type with cfg-based switching
  (ParseError when alloc enabled, MinimalError when not)
- Migrate all primitive/ and text/ parsers from `ParseError` to `I::Error`
- Gate alloc-dependent modules behind `#[cfg(feature = "alloc")]`:
  many, many1, sep_by, chainl1/r1, recursive, quoted_string, escaped
- Add `default = ["alloc"]` feature flag
- `cargo build --no-default-features` now succeeds for core-only usage
- Targets: RP2040/RP2350, WASM (no_std + no alloc)

### Features

* **parser:** add no_std core-only support without alloc ([a2a6e1b](https://github.com/j5ik2o/oni-comb-rs/commit/a2a6e1b405b005918c543be45e62669057a878ea))
* **parser:** unify collector combinators with fold-based primitives ([942604c](https://github.com/j5ik2o/oni-comb-rs/commit/942604c7a7e0723a7e5029436c74e85627252126))


### Bug Fixes

* **parser:** ungated chainl1 from alloc feature ([bd22eda](https://github.com/j5ik2o/oni-comb-rs/commit/bd22eda74170af10d77ddedb9b1ea7ccc9d99866))


### Performance Improvements

* **benches:** optimize JSON whitespace handling in benchmarks ([dd44ff6](https://github.com/j5ik2o/oni-comb-rs/commit/dd44ff62ca10cdc78ded6a02832472eddc6a1891))
* **parser:** reduce generic token decoding overhead ([463a190](https://github.com/j5ik2o/oni-comb-rs/commit/463a190ab0cde890dcf9a175e8b8afcafe36069d))

## [2.2.0](https://github.com/j5ik2o/oni-comb-rs/compare/oni-comb-parser-v2.1.0...oni-comb-parser-v2.2.0) (2026-03-17)


### Features

* **bench:** add pom full-json benchmark ([a31a407](https://github.com/j5ik2o/oni-comb-rs/commit/a31a40788e52db1de84ecd898d07660195f87a47))
* **parser:** add fold combinator types for zero-allocation repetition ([d482c46](https://github.com/j5ik2o/oni-comb-rs/commit/d482c46f59293c46cfa4c8970029d76411b131fc))
* **parser:** add fold/into methods to ParserExt ([b3da725](https://github.com/j5ik2o/oni-comb-rs/commit/b3da725b28db39b5cc3bcd50060329c7cfd683a6))
* **parser:** replace quoted_string_cow with quoted_string ([1f92d82](https://github.com/j5ik2o/oni-comb-rs/commit/1f92d8226fb4a93219b150a4f47f9fafdb92aee2))


### Bug Fixes

* migrate chumsky_impl.rs to chumsky 0.12 API ([2abb0a9](https://github.com/j5ik2o/oni-comb-rs/commit/2abb0a9e00cb63f8335db38185a1032fc1cece86))
* resolve merge conflict with dependabot/cargo/winnow-0.7 base branch ([2520981](https://github.com/j5ik2o/oni-comb-rs/commit/2520981e6490ed1fe8500e9e0d65386450de21a4))
* update winnow_impl.rs for winnow 0.7 API (ErrMode removed) ([6041236](https://github.com/j5ik2o/oni-comb-rs/commit/60412367a62e16d61ccc6d387a5bcd0b6af314fb))

## [2.1.0](https://github.com/j5ik2o/oni-comb-rs/compare/oni-comb-parser-v2.0.0...oni-comb-parser-v2.1.0) (2026-03-16)


### Features

* add `take` parser for consuming a fixed number of characters ([4f649ef](https://github.com/j5ik2o/oni-comb-rs/commit/4f649ef25aa2d1647d145a83034a77b0c086f984))
* add crond crate - cron expression parser using v2 combinator API ([fc1b68e](https://github.com/j5ik2o/oni-comb-rs/commit/fc1b68ed7176e4a6e76041e39e28cbeca1fdd65e))
* add map_res combinator for fallible value transformation ([6781aef](https://github.com/j5ik2o/oni-comb-rs/commit/6781aef74b4958f0f615bc0576d9bb145aeb50ab))
* add one_of and none_of generic parsers ([3e5201b](https://github.com/j5ik2o/oni-comb-rs/commit/3e5201be82fb1deaf870a2545908149cefdd7e66))
* add proptest round-trip tests, CI pipelines, and README for uri crate ([6153fc5](https://github.com/j5ik2o/oni-comb-rs/commit/6153fc58afe8a3e96ad371c5f2d3a015fc43b5f1))
* add regex parser behind optional `regex` feature flag ([a6679a0](https://github.com/j5ik2o/oni-comb-rs/commit/a6679a0e27cf7bae722fc5b5c132d700d0b6b2cb))
* add take_till0 and take_till1 generic parsers ([ead90c1](https://github.com/j5ik2o/oni-comb-rs/commit/ead90c1c3eaf7536b39d566597678b0f9f6ac965))
* add take_while_n_m parser for bounded character matching ([d694dff](https://github.com/j5ik2o/oni-comb-rs/commit/d694dffb862d5a6634850bacf451aa666f63f13d))
* add uri crate - RFC 3986 URI parser with zero-copy models and URN support ([711c8ad](https://github.com/j5ik2o/oni-comb-rs/commit/711c8ad06e9fa264bb2760c76bffc9af57cbac22))
* generify primitive parsers with `Input` trait and add `ByteInput` ([4b48931](https://github.com/j5ik2o/oni-comb-rs/commit/4b4893158ed052ff00d04edaca461464f1e2eeff))
* introduce generic `Input` trait and support for `ByteInput` ([b951c1e](https://github.com/j5ik2o/oni-comb-rs/commit/b951c1e467cef612fb071dfe6c616f5d4d7166c0))


### Bug Fixes

* accept empty port after colon per RFC 3986 ([ede6c21](https://github.com/j5ik2o/oni-comb-rs/commit/ede6c218f78a42e0b6e423a147db13108530c2b0))
* correct AnyStep evaluation for 1-based cron fields and prevent uint8 overflow panic ([479ecc9](https://github.com/j5ik2o/oni-comb-rs/commit/479ecc98b389f6c9c57474cfe15e6aab8336153b))
* correct Path::Absolute Display for bare `/` and URN NSS with slashes ([92de76d](https://github.com/j5ik2o/oni-comb-rs/commit/92de76da231b1fea9e1f0a12d2df12465345e05c))
* extend iterator search to 8 years and reject step=0 at parse time ([8d15158](https://github.com/j5ik2o/oni-comb-rs/commit/8d1515897501732fa0e483691de9390e01f8218a))
* include Recursive generification missed in initial commit ([8fdc665](https://github.com/j5ik2o/oni-comb-rs/commit/8fdc665485f22919e03a7721249696de015e8ef9))
* make urn_nss use path_rootless_segments consistent with urn_nid ([6261754](https://github.com/j5ik2o/oni-comb-rs/commit/626175453ebac75b595712e33bae9eabfe78fe20))
* reject leading zeros in IPv4 dec-octet and remove unused as_str_repr ([78fb579](https://github.com/j5ik2o/oni-comb-rs/commit/78fb5793580ec5b1370aede36254bf76d7b2a4d8))
* reject step=0 in Range evaluation instead of matching all values ([7d9b8a0](https://github.com/j5ik2o/oni-comb-rs/commit/7d9b8a02827ece219733a4f385272f2f3be480f7))
* remove unsafe in urn_nss and verify IPv4 is followed by delimiter ([f9e7de0](https://github.com/j5ik2o/oni-comb-rs/commit/f9e7de019e408aa318d95a633bf7df7c105604d5))
* resolve clippy type_complexity warning in lexeme return type ([504465c](https://github.com/j5ik2o/oni-comb-rs/commit/504465c06341fe3be31bc55bb11e806a18ed026c))
* resolve clippy type_complexity warnings in recursive and lexeme ([3a09a54](https://github.com/j5ik2o/oni-comb-rs/commit/3a09a5466886f50ac0159e5fc4a28a88626f885f))
* resolve dead_code warnings that fail CI with -D warnings ([775ae30](https://github.com/j5ik2o/oni-comb-rs/commit/775ae30bccad3cd8568bcaa65b10d3ce4e66db22))
* support wrap-around ranges and truncate sub-minute precision ([d2f41e0](https://github.com/j5ik2o/oni-comb-rs/commit/d2f41e02d50ca9e22a3bbf1c506cdd0e69c89036))
* wrap regex build error in own RegexBuildError type ([4a2c9fb](https://github.com/j5ik2o/oni-comb-rs/commit/4a2c9fb5cdeec4e4713401a4b5acce861c008892))

## [2.0.0](https://github.com/j5ik2o/oni-comb-rs/compare/5a8b91c73e72c33ac41b80ac5979011ec73a44dc...oni-comb-parser-v2.0.0) (2026-03-16)


### ⚠ BREAKING CHANGES

* All text parsers now use ParseError instead of String
for their Error associated type. This provides:

- Position tracking (byte offset of failure)
- Expected token sets (Char, Tag, Description, Eof)
- Context stacking via .context() combinator
- Automatic merging of expected sets in `or` combinator
- Display impl for human-readable error messages

The or combinator now requires MergeError bound on Error type.
The .context() method requires ContextError bound.

Expected enum includes Byte/ByteTag variants reserved for future
bytes input support.
* .then() is removed, use .zip() instead (Applicative product)

- Rename Then<P1,P2> to Zip<P1,P2>, then.rs to zip.rs, .then() to .zip()
- Rewrite README: "parser monad library" with full type class hierarchy
  (Functor/Applicative/Alternative/Monad), cost table, and Applicative-first rationale
- Add type class column to combinator method table in README
- Update CLAUDE.md to reflect zip/flat_map/and_then naming
* 🧨 Scheme to Option<Scheme>

### Features

* &charに対するElementトレイトの実装を追加 ([465238f](https://github.com/j5ik2o/oni-comb-rs/commit/465238fdc1fbd6c6a08e8995c4aaa0c89a6f7738))
* 🎸 add methods to Uri ([44022cb](https://github.com/j5ik2o/oni-comb-rs/commit/44022cb9f505d925aa14d525639cd6405470d9cc))
* 🎸 add the method to Uri ([b4d624d](https://github.com/j5ik2o/oni-comb-rs/commit/b4d624d9398d88a30b2517c672ca414fbef7f3cf))
* 🎸 Change scheme to optional ([af44225](https://github.com/j5ik2o/oni-comb-rs/commit/af44225a3962ed9cf219efdb07c98fd21f39ad90))
* add `okite-ai` reference file ([27e3568](https://github.com/j5ik2o/oni-comb-rs/commit/27e35682be7937d45a1f399978c469c94e679ab2))
* add flat_map/and_then (monadic bind) and Parser impl for Box<dyn Parser> ([dda5e17](https://github.com/j5ik2o/oni-comb-rs/commit/dda5e1721c985f5cd56a221848c1abb4931fe682))
* add new parser combinators and corresponding tests ([1014976](https://github.com/j5ik2o/oni-comb-rs/commit/101497659d07c183b37cee0bd9c8fe90c9229890))
* add no_std support with alloc, add feature comparison table ([c09098b](https://github.com/j5ik2o/oni-comb-rs/commit/c09098bb8e9c1273a2bc0c770bec4c403633282d))
* add recursive() helper for recursive parser construction ([c2d5839](https://github.com/j5ik2o/oni-comb-rs/commit/c2d583995b1d0b67c39cb585e250eba9e9fd68e4))
* add support for edtion2021 ([5a8b91c](https://github.com/j5ik2o/oni-comb-rs/commit/5a8b91c73e72c33ac41b80ac5979011ec73a44dc))
* add tasks for code formatting and dependency sorting ([a92ead5](https://github.com/j5ik2o/oni-comb-rs/commit/a92ead52b399bab9e018bdabadaf0e49cb44cc8e))
* add text module parsers (whitespace, identifier, integer, quoted_string, escaped, lexeme) ([80d6b1d](https://github.com/j5ik2o/oni-comb-rs/commit/80d6b1d90f60ae454ff503614526f50d86f29456))
* add text parsers (satisfy, take_while) and benchmark suite ([1f627e6](https://github.com/j5ik2o/oni-comb-rs/commit/1f627e6554983f73e6e66ac697d2bdcd67240f3d))
* **bench:** add flat_map benchmarks across 5 parser libraries ([7424b97](https://github.com/j5ik2o/oni-comb-rs/commit/7424b97d6915e4f107f2e39a5401224e2a5d470e))
* **bench:** add JSON/arithmetic benchmarks, complete MS7 ([47928b8](https://github.com/j5ik2o/oni-comb-rs/commit/47928b81057228319eefe45f8e8b321a94717cf7))
* bootstrap parser crate skeleton ([56ad127](https://github.com/j5ik2o/oni-comb-rs/commit/56ad1271d02ad4600c25eb515cabbfcd47b48d77))
* complete Phase 1 of StaticParser implementation ([092c8eb](https://github.com/j5ik2o/oni-comb-rs/commit/092c8ebf2a2afbe3bd13e6a98263ad25148757e9))
* deprecate to_static_parser() function and optimize StaticParser implementation ([f4cf6c4](https://github.com/j5ik2o/oni-comb-rs/commit/f4cf6c47cfce24d273d536a25b8d88ad67358f6a))
* establish core parser state and result scaffolding ([810adbd](https://github.com/j5ik2o/oni-comb-rs/commit/810adbd670a0d0ab0027f2b15642a5a95bc6bc48))
* implement direct StaticParser benchmarks for issue [#1021](https://github.com/j5ik2o/oni-comb-rs/issues/1021) ([a0aca55](https://github.com/j5ik2o/oni-comb-rs/commit/a0aca551c9078b60ba112354fee7be04262c392a))
* implement remaining StaticParser implementations for Phase 1 migration ([52dc596](https://github.com/j5ik2o/oni-comb-rs/commit/52dc596e87e5f9f80275f2cd8b06f6dc47374591))
* implement StaticParser for improved performance ([157aca6](https://github.com/j5ik2o/oni-comb-rs/commit/157aca62c3f8dda9108d3097c5f222b0f869685f))
* implement StaticParser module structure for Phase 2 migration ([fc4b75b](https://github.com/j5ik2o/oni-comb-rs/commit/fc4b75bdf23a3864bf7a0b4a7aac813ac35facd1))
* implement StaticParser traits and operators for Phase 1 migration ([7497061](https://github.com/j5ik2o/oni-comb-rs/commit/74970612e928d56ef0cce122e5b6466b728a37e7))
* improve JSON parser benchmarks with various test data and fix parser implementations ([1800a3d](https://github.com/j5ik2o/oni-comb-rs/commit/1800a3d73b5c29a7478671faf1d0cfa7cde8ef5f))
* integrate OpenSpec command framework and add support for experimental workflows ([08c18da](https://github.com/j5ik2o/oni-comb-rs/commit/08c18da0870bd72f62b21b19ffb8ab613d04c598))
* migrate examples to use direct StaticParser implementation for issue [#1015](https://github.com/j5ik2o/oni-comb-rs/issues/1015) ([823297e](https://github.com/j5ik2o/oni-comb-rs/commit/823297e6b44dc2529ce4e2778f6e9ed8e4ae7186))
* ParseResultとParserの基本機能を拡充 ([9978bf2](https://github.com/j5ik2o/oni-comb-rs/commit/9978bf2ea58eede2cdc6ffa595797eff40aab257))
* replace String errors with structured ParseError ([728869e](https://github.com/j5ik2o/oni-comb-rs/commit/728869ec160f8ceae11d70d56294d66d51e9d419))
* StaticParserにof_many1とcollectメソッドを追加 ([adccead](https://github.com/j5ik2o/oni-comb-rs/commit/adccead67e7c57cf0b10372d60fdb51eab0a224c))
* Support for strings without quotes ([0f2e7a9](https://github.com/j5ik2o/oni-comb-rs/commit/0f2e7a963d0b4717ca3fefa3d1913573df53f33c))
* パーサーコンビネータの基本実装を追加 ([c4c47ec](https://github.com/j5ik2o/oni-comb-rs/commit/c4c47ec1b82199ca6dce6ab67c57c69fe69f275b))
* パーサーメソッドチェーンAPIを拡充 ([471a2fc](https://github.com/j5ik2o/oni-comb-rs/commit/471a2fc939d493e599f5356ff68a8a6aa814f467))


### Bug Fixes

* 🐛 reg-name parsing bug ([935db4f](https://github.com/j5ik2o/oni-comb-rs/commit/935db4f51579f8b1c5fd6bc29dd4df6e0f84b76e))
* build error ([ec27b79](https://github.com/j5ik2o/oni-comb-rs/commit/ec27b796b72f58df1de507ae96b33f34c61cc1c6))
* **deps:** update rust crate anyhow to 1.0.65 ([48d92c1](https://github.com/j5ik2o/oni-comb-rs/commit/48d92c14a5affd9ee0c5df61d3ef02d243372216))
* **deps:** update rust crate regex to 1.6 ([399eaa2](https://github.com/j5ik2o/oni-comb-rs/commit/399eaa2700e1bca01dcb4b09f6c0163b38fd59e5))
* **deps:** update rust crate regex to 1.8 ([01cc041](https://github.com/j5ik2o/oni-comb-rs/commit/01cc0410b216541e25ef9958f40bcb3038ab1580))
* **deps:** update rust crate regex to 1.9 ([274078f](https://github.com/j5ik2o/oni-comb-rs/commit/274078f1cf0ebbd40f9ada304d520f05f7147e50))
* **deps:** update rust crate rust_decimal_macros to 1.29 ([2b01023](https://github.com/j5ik2o/oni-comb-rs/commit/2b010233f2bd6bd7e2d053cab52421291ca5b6eb))
* **deps:** update rust crate rust_decimal_macros to 1.30 ([a210c50](https://github.com/j5ik2o/oni-comb-rs/commit/a210c500669adba3c2fa00d9826a16a19dcc5859))
* **deps:** update rust crate rust_decimal_macros to 1.31 ([f4e9817](https://github.com/j5ik2o/oni-comb-rs/commit/f4e9817df3f3600501a9803f9915f06d1221339f))
* **deps:** update rust crate rust_decimal_macros to 1.32 ([54add07](https://github.com/j5ik2o/oni-comb-rs/commit/54add077b7c051071fcde2c7d46ebb6961354b8b))
* elm_multi_space_ref_staticの戻り値の型を修正 ([3524da9](https://github.com/j5ik2o/oni-comb-rs/commit/3524da93a7aa869f9e1c9b8e9ec83e668b10d4fa))
* elm_multi_space_staticの戻り値の型を修正 ([faa871b](https://github.com/j5ik2o/oni-comb-rs/commit/faa871b5cf27b12163bf1f4b47362b50bef56e38))
* hiper_part parsing bug ([04f66de](https://github.com/j5ik2o/oni-comb-rs/commit/04f66de126908a2af623e04aa600936659405191))
* improve benchmark script and remove nom_json and pom_json imports ([e66051b](https://github.com/j5ik2o/oni-comb-rs/commit/e66051b5af2bcf162907883c02824fda4c051c7c))
* reformat ([8bf37fc](https://github.com/j5ik2o/oni-comb-rs/commit/8bf37fc2d836c031c4275d9875d017717432f7fa))
* remove nom_json and pom_json imports from bench_main.rs ([b2ebb18](https://github.com/j5ik2o/oni-comb-rs/commit/b2ebb18f85421512fd8e83ffbbfc3b386d4dc4e0))
* resolve type bounds in StaticParser implementation ([1527c15](https://github.com/j5ik2o/oni-comb-rs/commit/1527c15ca53971cbbe8c2df82ed444ba13f76808))
* revert optimization changes that caused performance regression ([1d46e53](https://github.com/j5ik2o/oni-comb-rs/commit/1d46e537ddf24d8c2515726fd40881dcf4ee63ae))
* specify type parameters for HashMap in cache_parsers_impl.rs ([03b82d4](https://github.com/j5ik2o/oni-comb-rs/commit/03b82d46ab0db734f6571a8d7a5e52f67d65dd61))
* StaticParserのcollectメソッドを修正 ([085140b](https://github.com/j5ik2o/oni-comb-rs/commit/085140b61fc441c146a81f613d22c889323c4116))
* StaticParserのエラーハンドリングを修正 ([efa6724](https://github.com/j5ik2o/oni-comb-rs/commit/efa672448b6271a13a85c0c744207b0dbee50f05))
* StaticParserのテストでParserをクローンするように修正 ([b6c0bbf](https://github.com/j5ik2o/oni-comb-rs/commit/b6c0bbfcefe86a1231299bcd406495e85e5866ab))
* StaticParserのテストの型注釈を追加 ([eba2a15](https://github.com/j5ik2o/oni-comb-rs/commit/eba2a158a8769701c519684d248631e3650603b1))
* StaticParserのテストを修正 ([156434b](https://github.com/j5ik2o/oni-comb-rs/commit/156434ba37b9c2698d899b4a6ea7cb9e26f2f773))
* StaticParserのテスト入力型を修正 ([068d155](https://github.com/j5ik2o/oni-comb-rs/commit/068d15570352a18e50a7db1c28bd2ac4d6b8ce72))
* StaticParserの実装を修正してフォーマットを整える ([49c9ff4](https://github.com/j5ik2o/oni-comb-rs/commit/49c9ff458c33811c3fd2c28fd3247d59438104ca))
* StaticParserの実装を修正して型エラーを解消 ([b6aeda7](https://github.com/j5ik2o/oni-comb-rs/commit/b6aeda73f239a6a908d5e15f4ab4873d9c8db310))
* StaticParserの実装を修正して型エラーを解消 ([c941717](https://github.com/j5ik2o/oni-comb-rs/commit/c94171719ae230feb7fa8ab228747f13e9c8a13c))
* StaticParserの実装を修正して型エラーを解消 ([46c3893](https://github.com/j5ik2o/oni-comb-rs/commit/46c38932951f11a9b2bff7a492aa700c38177382))
* StaticParserの実装を修正して型エラーを解消 ([b4ff097](https://github.com/j5ik2o/oni-comb-rs/commit/b4ff09785d1cf1098c47e6157c0e7f3af291db43))
* StaticParserの実装を修正して型エラーを解消 ([a3d223d](https://github.com/j5ik2o/oni-comb-rs/commit/a3d223d2a6351a536cd89d8dd89a165fa50347dd))
* StaticParserの実装を修正して型エラーを解消 ([bd00a9b](https://github.com/j5ik2o/oni-comb-rs/commit/bd00a9bee16415a5a9fec0f125162d1125808323))
* StaticParserの実装を修正して型エラーを解消 ([3e50c3c](https://github.com/j5ik2o/oni-comb-rs/commit/3e50c3c6bb915904ab704c5273f9a9d8558e9f92))
* StaticParserの実装を修正して型エラーを解消 ([dad66cb](https://github.com/j5ik2o/oni-comb-rs/commit/dad66cbd0fb10266fa5ea7dc5e3f7c57038c0328))
* surroundの実装を修正してdoctestを修正 ([8f3eb32](https://github.com/j5ik2o/oni-comb-rs/commit/8f3eb32ec72cf1bc3f793a070e24d65413ddf1a1))
* toolchain channel ([1281cfe](https://github.com/j5ik2o/oni-comb-rs/commit/1281cfe751a890d8670f189b8109a339616f4d40))


### Performance Improvements

* 6x speedup on 107KB JSON bench — surpass winnow ([d566945](https://github.com/j5ik2o/oni-comb-rs/commit/d5669456fc4fe1baed6c2d5c4f3641870a064744))
* add #[inline] to all parse_next implementations ([569b0e4](https://github.com/j5ik2o/oni-comb-rs/commit/569b0e4c62f8d4afa620032a251bdb35df550917))
* add take_while1_fold and refine benchmarks ([a2896ae](https://github.com/j5ik2o/oni-comb-rs/commit/a2896ae8e601a305a7159b6bfa97502a09c3a283))
* improve JSON parser performance with caching and memory optimization ([1bb76bf](https://github.com/j5ik2o/oni-comb-rs/commit/1bb76bf36ac27a3073d49d5e015cde3c243c2673))
* introduce take_while1 and CSV tuning ([b781641](https://github.com/j5ik2o/oni-comb-rs/commit/b7816419a33d2771c690fb33ea019c67457cc01e))
* optimize parser library for better performance ([0be897e](https://github.com/j5ik2o/oni-comb-rs/commit/0be897e3d204af62847c80957aa12545bd64907a))
* ParseCursorを導入してRcクローンを削減 ([7a038a1](https://github.com/j5ik2o/oni-comb-rs/commit/7a038a183e758d74bfac547eae63a1050aaa28c3))
* ParserのRcクローンを削減 ([bc250f2](https://github.com/j5ik2o/oni-comb-rs/commit/bc250f28431a7ed88ac821dc642de9127af9dac0))
* Parserを静的ディスパッチ可能な内部構造に再設計 ([84aecf1](https://github.com/j5ik2o/oni-comb-rs/commit/84aecf1a7ab45413ac8184dc7a2805e043c9ebb7))
* ParseStateで残り入力を参照として保持 ([ec8c89d](https://github.com/j5ik2o/oni-comb-rs/commit/ec8c89d1ac71c007d0c43867be11d739863a78ff))


### Code Refactoring

* rename then→zip, rewrite README as parser monad library ([b0562f3](https://github.com/j5ik2o/oni-comb-rs/commit/b0562f34312c2bb4620c8742c53769792af6795a))

