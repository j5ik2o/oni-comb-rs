## Why

`recursive()` のランタイム改善で算術系の再帰コストは下がったが、`json_full` はなお `or` 連鎖による checkpoint/reset と branch dispatch の税金を強く受けている。手書き `parse_next` や `fn_parser` に逃げずに、public combinator chain を維持したままこのコストを下げるには、先頭 token/byte から候補を絞る declarative な predictive choice combinator が必要である。

## What Changes

- 先頭 token/byte の観測結果に基づいて分岐先 parser を選ぶ predictive choice combinator を追加する
- 分岐選択は入力を消費せずに行い、通常の `or` のような逐次 checkpoint/reset 連鎖を必須にしない
- public combinator chain の書き味を維持したまま、JSON の value choice のような先頭文字で強く予測できる grammar を declarative に書けるようにする
- `StrInputStream` / `ByteInputStream` で使いやすい fast-path API としつつ、誤予測時の Backtrack/Cut semantics は既存 parser contract と整合させる
- benchmark 用 JSON 実装など、predictive choice の恩恵が大きい箇所に適用できるよう設計する

## Capabilities

### New Capabilities

- `predictive-choice`: 先頭 token/byte に基づく declarative な choice combinator と、その失敗意味論・入力非消費の契約

### Modified Capabilities

<!-- None -->

## Impact

- `modules/parser/src/combinator/`: 新しい predictive choice combinator 型の追加対象
- `modules/parser/src/parser_ext.rs` または text/byte 向け facade: 利用しやすい public API の追加対象
- `modules/parser/src/str_input_stream.rs`, `modules/parser/src/byte_input_stream.rs`: 先頭 byte fast path の利用点になりうる
- `modules/parser/benches/workloads/json.rs`, `modules/parser/benches/json_full.rs`, `modules/json/src/parser.rs`: 適用候補
- benchmark / README: `or` 連鎖ボトルネックの更新対象
- public API: additive change のみで、破壊的変更は想定しない
