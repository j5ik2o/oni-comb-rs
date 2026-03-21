## Context

`recursive()` は現在 [modules/parser/src/combinator/recursive.rs](/Users/j5ik2o/Sources/j5ik2o.github.com/j5ik2o/oni-comb-rs/modules/parser/src/combinator/recursive.rs) で `Rc<UnsafeCell<Option<Box<dyn Parser>>>>` を使って自己参照を実現している。この設計は API としては小さいが、`Recursive::parse_next` の steady-state に `Option` unwrap と trait object dispatch を残すため、再帰文法の hot path で不利である。README のベンチ分析でも、算術パーサや full JSON の遅さの一因として `recursive()` の間接呼び出しコストが明示されている。

一方で、このプロジェクトでは downstream parser を public combinator chain だけで記述する方針が強い。JSON や YAML のような grammar は `recursive(|value| { ... value.clone() ... })` の declarative な書き方を維持する必要があり、`fn_parser`、manual `parse_next`、手書き dispatch に戻す方向は採らない。したがって、改善対象は grammar authoring layer ではなく `recursive()` の内部ランタイムである。

## Goals / Non-Goals

**Goals:**
- `recursive()` の public API、型シグネチャ、`Clone` による再利用パターンを維持する
- steady-state の `Recursive::parse_next` から `Box<dyn Parser>` と `Option` unwrap を外し、hot path を単純化する
- root owner と parser graph 内の自己参照 handle を分離し、強参照サイクルを作らない内部構造へ移行する
- JSON / YAML / arithmetic のような既存 recursive grammar が変更なしで動作することを保つ
- unsafe を使う場合でも drop 順序、clone、未初期化参照の条件を明文化し、テストとベンチで検証可能にする

**Non-Goals:**
- `recursive()` を `fn_parser` ベース API へ置き換えること
- JSON / YAML parser を imperative に書き換えること
- `Parser` trait 全体の再設計
- 再帰文法のあらゆる間接呼び出しをゼロにすること
- `recursive()` 以外の combinator hot path を同 change で最適化すること

## Decisions

### D1. `recursive()` は owner handle と self-reference handle を分離する

`recursive()` が返す root parser は allocation の所有権を持つ owner handle とし、closure に渡す `Recursive` clone は parser graph 内でのみ使われる non-owning self-reference handle とする。これにより parser graph に埋め込まれた自己参照が `Rc` の強参照サイクルを形成しないようにする。

- Why: `Recursive` が一様に `Rc` owner を持つと、`inner -> parser graph -> Recursive clone -> inner` の循環が発生しやすい
- Alternative considered: すべての `Recursive` clone に `Rc` owner を持たせ続ける
- Why not: ownership が曖昧になり、drop 不能な cycle を作りやすい
- Alternative considered: `Weak` を parser graph 内参照に使う
- Why not: hot path に `upgrade()` 相当のコストや失敗分岐を持ち込みたくない

### D2. steady-state dispatch は `Box<dyn Parser>` ではなく typed thunk を使う

内部表現は `Box<P>` のような具体型 storage とし、`Recursive` は `data_ptr` と `parse_fn` / `drop_fn` の thunk を保持する。`Recursive::parse_next` は `parse_fn(data_ptr, input)` を呼ぶだけの形にし、trait object dispatch を除去する。

- Why: 主要なボトルネックは `dyn Parser` による vtable dispatch と最適化阻害であり、ここを外すのが最も効果的
- Alternative considered: `Box<dyn Parser>` のまま `#[inline(always)]` や局所最適化で凌ぐ
- Why not: hot path の支配的コストを温存したままになる

### D3. 初期化フェーズと実行フェーズを内部的に分け、steady-state から `Option` を外す

構築中だけ未初期化を許容し、`f(rec_ref)` で parser graph を組み立てたあとに runtime slot を fully initialized state へ遷移させる。steady-state の parse では `Option<Box<...>>` を見ない。

- Why: `recursive()` は初期化後に空状態へ戻らないため、実行パスに `Option` を残す理由がない
- Alternative considered: 現状の `Option` を残して `expect()` で守る
- Why not: steady-state に不要な分岐を残し、ホットパスを太らせる

### D4. `Clone` の意味論は維持しつつ、owner の有無だけを内部状態として分ける

利用者から見た `Recursive: Clone` は変えない。root owner の clone も graph 内 self-ref の clone も同じ `Recursive` 型で扱うが、内部フラグまたは field 構成で owner の有無を区別する。graph 内 clone は ownership を増やさず、root owner clone は allocation lifetime を保持する。

- Why: JSON / YAML / arithmetic は `value.clone()` を前提に組み立てており、ここを壊すと grammar 側の変更が必要になる
- Alternative considered: owner 用と ref 用の別 public 型に分ける
- Why not: public API 変更になり、既存 grammar を壊す

### D5. 検証は意味論回帰と recursive-heavy benchmark の両方で行う

`modules/parser/tests/recursive.rs`、`modules/parser/tests/arithmetic.rs`、JSON/YAML の compile/runtime 使用箇所を意味論回帰として扱い、bench では arithmetic と recursive-heavy JSON を確認対象にする。ベンチの目標は `fn_parser` 並みではなく、`recursive()` の既存税を明確に減らすことである。

- Why: 今回の change の価値は API 互換だけでなく、再帰文法の hot path 改善にある
- Alternative considered: テストのみで完了し、性能確認は後続 change へ送る
- Why not: `recursive()` 改善が本当に効いたか判断できない

## Risks / Trade-offs

- [Risk] `unsafe` な typed thunk 実装で型整合性や drop 順序を壊すと UB になる → Mitigation: `parse_fn` / `drop_fn` の生成を単一箇所に閉じ込め、owner/ref lifetime 条件を設計文書とテストで固定する
- [Risk] owner/ref 分離が複雑で clone semantics のバグを生む → Mitigation: root owner clone、graph 内 clone、nested recursive grammar の3系統を専用テストで押さえる
- [Risk] thunk dispatch にしても indirect call 自体は残るため期待より改善しない → Mitigation: 目標を「trait object と steady-state 分岐の除去」に置き、ベンチで実測して次段の施策を判断する
- [Risk] 構築中と実行中の状態遷移が曖昧だと未初期化 parse を許してしまう → Mitigation: 初期化前は到達不能な sentinel を使うか、構築 API 内でのみ未初期化状態を閉じ込める

## Migration Plan

1. `recursive()` の現行実装と利用パターン（JSON / YAML / arithmetic）を固定し、必要な意味論回帰テストを明確にする
2. owner/ref 分離と typed thunk dispatch の内部型を設計し、steady-state から `dyn Parser` と `Option` を外す
3. `Clone` / drop / nested recursion の回帰テストを追加または更新する
4. arithmetic と recursive-heavy benchmark を再計測し、README のボトルネック記述を更新する
5. 性能改善が不十分な場合は、次段として predictive choice や利用側 grammar の fold 化を別 change で検討する

## Open Questions

- initialized sentinel を panic thunk にするか、構築 API の型で未初期化状態を隠すか
- `data_ptr` は `*mut ()` と `NonNull<()>` のどちらで保持するか
- ベンチの完了条件を絶対値ではなく改善率で置くか、代表 workload の mean で置くか
