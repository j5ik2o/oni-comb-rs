## Context

oni-comb-rs v2 は trait + concrete combinator 型による Applicative/Alternative 主体の設計を採用しており、`then`, `or`, `map` 等のコンビネータ合成でヒープアロケーションがゼロであることをベンチマークで実証済み。しかし、1つ目のパーサーの結果に基づいて次のパーサーを動的に選択するモナディック合成（`flat_map`）が未提供であり、文脈依存文法や動的分岐を表現できない。

既存のコンビネータ型（`Map`, `Then`, `Or`, `Attempt`, `Cut`, `Optional`, `Many`）は `parser/src/combinator/` に格納され、それぞれ `Parser` トレイトを実装する concrete 型として定義されている。`ParserExt` トレイト（`parser/src/parser_ext.rs`）がメソッドチェーン API を提供する。

## Goals / Non-Goals

**Goals:**
- `ParserExt` に `.flat_map(f)` メソッドを追加し、モナディックパーサー合成を可能にする
- クロージャの戻り値が同一型なら `Box` 不要で concrete 型のまま使えるようにする
- 異なる型を返す場合は `Box<dyn Parser>` による型消去をユーザーが明示的に選択する
- Fail セマンティクス（Backtrack / Cut / Incomplete / ZeroProgress）は既存コンビネータと一貫させる
- テストで flat_map の基本動作・エラー伝播・他コンビネータとの組み合わせを検証する

**Non-Goals:**
- `do` 記法やマクロによるモナディック DSL の提供（将来の拡張として別途検討）
- `flat_map` を使った再帰パーサー（Milestone 5 の `recursive()` ヘルパーで対応）
- 性能最適化（まず正しい実装を優先。最適化は Milestone 7 で計測後に判断）

## Decisions

### 1. `FlatMap<P, F>` を concrete combinator 型として実装

**選択**: `combinator/flat_map.rs` に `FlatMap<P, F>` 構造体を定義し、`Parser` トレイトを実装する。

**理由**: 既存コンビネータ（`Map<P, F>`, `Then<P1, P2>` 等）と同じパターンに従い、一貫性を保つ。`F` の戻り値型がパーサーであれば、そのパーサーの `parse_next` を呼ぶだけでよい。

**代替案**: トレイトオブジェクト（`Box<dyn Parser>`）を内部的に常に使用する方式 → 不要なアロケーションが発生するため却下。

### 2. 型消去はユーザー側の責任

**選択**: `flat_map` 自体は型パラメータ `P2` を要求するだけで、`Box` 化を強制しない。異なる型を返す場合はユーザーが `Box<dyn Parser>` を使う。

**理由**: 全分岐が同じ型を返すケース（例: 全分岐が `Tag` を返す）では `Box` は不要であり、ゼロコストを維持できる。型消去が必要かどうかはユーザーが判断すべき。

```rust
// Box 不要（全分岐が Tag）
satisfy(|c| c.is_digit(10)).flat_map(|_| tag("num"))

// Box 必要（分岐ごとに異なる型）
satisfy(|c| c.is_digit(10)).flat_map(|n| -> Box<dyn Parser<...>> {
    match n { '1' => Box::new(tag("one")), _ => Box::new(char('x')) }
})
```

### 3. Fail セマンティクスは透過的伝播

**選択**: 1つ目のパーサーの Fail はそのまま返す。成功した場合、`f` が返したパーサーの Fail もそのまま返す。`flat_map` 自体は Fail の変換を行わない。

**理由**: `then` と同じ原則。Fail の制御は `attempt`, `cut` 等の専用コンビネータの責務であり、`flat_map` が勝手に変換すると予測しづらくなる。

## Risks / Trade-offs

- **[Risk] `Box<dyn Parser>` の Error 型の統一** → `flat_map` のクロージャが返すパーサーの `Error` 型は `P` の `Error` 型と一致する必要がある。型制約で強制する。
- **[Risk] パフォーマンス退行のリスク** → `flat_map` はユーザーが明示的に使うものであり、既存の Applicative コンビネータには影響しない。ベンチマーク回帰テストで確認可能。
- **[Trade-off] `Box<dyn Parser>` 使用時のライフタイム制約** → `StrInput<'a>` のライフタイムと `Box<dyn Parser>` の関係が複雑になりうる。テストで実用パターンを検証する。
