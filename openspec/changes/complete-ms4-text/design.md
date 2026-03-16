# MS4 Text module — 設計

## 設計方針

- whitespace / identifier / integer は既存コンビネータの**合成**で実装。新しい具象型は作らない
- quoted_string だけは**専用プリミティブ**。エスケープ処理をループ内で行うため、コンビネータ合成では非効率
- すべてコンストラクタ関数として提供（`whitespace0()`, `identifier()` 等）

## 各パーサーの設計

### whitespace0 / whitespace1

```rust
pub fn whitespace0() -> TakeWhile0<fn(char) -> bool> {
    take_while0(|c: char| c.is_ascii_whitespace())
}

pub fn whitespace1() -> TakeWhile1<fn(char) -> bool> {
    take_while1(|c: char| c.is_ascii_whitespace())
}
```

返り値は `&str`（消費した空白文字列）。新しい型は不要。

### identifier

```rust
pub fn identifier() -> impl Parser<StrInput<'_>, Output = String, Error = String>
```

内部: `satisfy(alpha|_).zip(take_while0(alnum|_)).map(結合)`

返り値は `String`（先頭文字 + 残りを結合）。

### integer

```rust
pub fn integer() -> impl Parser<StrInput<'_>, Output = i64, Error = String>
```

内部:
- optional な `-` 符号
- `take_while1(digit)`
- `.map()` で `i64` に変換

### quoted_string — 専用プリミティブ

```rust
pub struct QuotedString;

pub fn quoted_string() -> QuotedString { QuotedString }
```

`Parser<StrInput<'a>>` を直接実装。ループ内で以下を処理:

1. 開始 `"` を消費
2. ループ:
   - `\` → エスケープシーケンスを解釈
   - `"` → 終了
   - その他 → そのまま push
3. `String` を返す

#### JSON 準拠エスケープ一覧

| シーケンス | 出力 |
|-----------|------|
| `\"` | `"` |
| `\\` | `\` |
| `\/` | `/` |
| `\b` | `\x08` (BS) |
| `\f` | `\x0C` (FF) |
| `\n` | `\n` |
| `\r` | `\r` |
| `\t` | `\t` |
| `\uXXXX` | Unicode コードポイント（サロゲートペア非対応、将来拡張） |

不正なエスケープ → `Fail::Cut`（`"` を消費済みなのでバックトラック不可）。

#### Fail 意味論

```
"..." の途中で入力が尽きた → Fail::Cut（開始 " を消費済み）
不正なエスケープ           → Fail::Cut
\uXXXX の桁数不足          → Fail::Cut
開始 " がない               → Fail::Backtrack
```

## JSON subset 統合テスト

MS4 完了条件の実証として、以下の JSON subset パーサーをテストに記述:

```rust
// JSON value = null | bool | integer | string | array | object
// null:   tag("null")
// bool:   tag("true").or(tag("false"))
// number: integer()
// string: quoted_string()
// array:  between(char('['), value.sep_by0(char(',')), char(']'))
// object: between(char('{'), pair.sep_by0(char(',')), char('}'))
// pair:   quoted_string().zip_left(char(':')).zip(value)
// ws:     各トークンの前後に whitespace0()

// テスト入力例:
// {"name": "oni-comb", "version": 2, "features": ["fast", "safe"], "active": true}
```

再帰（value が array/object を含む）は MS5 の `recursive()` なしでは直接書けない。
テストでは **1段のネスト**（配列やオブジェクトの値がプリミティブのみ）で検証する。
