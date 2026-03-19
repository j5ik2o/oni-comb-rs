## Context

oni-comb の JSON ベンチは、マクロ性能ではすでに `fn_parser`、`peek_byte`、ゼロコピー文字列・数値で大きく前進している。一方で `modules/parser/benches/json_full.rs` と `modules/parser/benches/alloc_count.rs` の手書き JSON パーサーは、`json_value()`、`json_array()`、`json_object()`、`json_member()` の複数箇所で `whitespace0()` を繰り返し呼んでおり、同じ grammar boundary で空白を再走査しやすい。

`modules/parser/benches/workloads/json.rs` でも `ws(p) = whitespace0().zip_right(p).zip_left(whitespace0())` を多用しており、値・区切り記号・括弧をすべて対称ラップしている。この構造は分かりやすい反面、JSON subset ベンチの fixed cost を押し上げる。今回の変更は公開 API を変えず、ベンチ用 JSON 実装の空白責務だけを整理する。

## Goals / Non-Goals

**Goals:**
- JSON フルベンチと allocation-count 用 JSON パーサーで、空白消費を grammar boundary ごとに一度だけ行う構造に整理する
- JSON subset ベンチでも、不要な `ws()` ラップを減らして同じ JSON 受理範囲を保つ
- `json_full.rs` と `alloc_count.rs` の whitespace policy を同期させ、性能計測とアロケーション計測の比較可能性を維持する
- ベンチ README に、空白固定コスト改善の意図と結果を反映する

**Non-Goals:**
- JSON 文法そのものの拡張（comments, trailing commas, Unicode whitespace など）
- `oni_comb_parser` の公開 API や combinator 意味論の変更
- `recursive()` や `flat_map` の別ボトルネック解消
- 他ワークロード（identifier/integer/arithmetic など）の最適化

## Decisions

### D1: 空白処理を boundary-scoped helper に寄せる

**選択**: 手書き JSON パーサーでは、`json_value()` が常に先頭空白を食べる構造をやめ、`value entry` と `value body` の責務を分ける。配列・オブジェクト・メンバーの区切り判定も、「区切り記号や閉じ括弧を確認する直前に一度だけ空白を飛ばす」形に統一する。

**理由**: 現状は `json_array()` / `json_object()` が空白を消費した直後に、子の `json_value()` / `json_member()` が再度 `whitespace0()` を呼ぶ経路がある。空白処理の責務を caller / callee のどちらかに固定すれば、同じ grammar boundary を二重に走査しにくくなる。

**代替案**:
- 現状維持で `whitespace0()` のインライン化に期待する
  - 空白スキャン回数そのものは減らないため、fixed cost 改善が限定的
- すべて `lexeme()` 風 combinator に置き換える
  - 手書きの `peek_byte` 分岐や `fn_parser` 構成を崩しやすく、full JSON の速さを損なうリスクがある

### D2: `json_full.rs` と `alloc_count.rs` は同じ oni-comb JSON 実装を共有する

**選択**: oni-comb 側の full JSON パーサー本体はベンチ共通の補助モジュールへ寄せ、`json_full.rs` と `alloc_count.rs` はそれを使う。

**理由**: すでに `alloc_count.rs` 側に「json_full.rs と同一」とコメントがあるが、実体はコピーであり、最適化時のズレが起きやすい。空白処理のような細かいホットパス調整は、実装が二重化していると README・ベンチ結果・allocation profile の整合性を崩しやすい。

**代替案**:
- 2ファイルを個別に直してコメントで同期を保つ
  - 今回のような micro-opt のたびに差分が発生しやすく、保守コストが高い
- `alloc_count.rs` だけ別実装のままにする
  - 計測対象が揃わず、性能とアロケーションの説明が弱くなる

### D3: JSON subset ベンチでは `ws(p)` の全面ラップをやめ、トークン別 helper に分割する

**選択**: `workloads/json.rs` の `ws(p)` をそのまま全トークンに適用する方式から、`value`, `comma`, `colon`, `open/close delimiter` ごとに空白責務を分けた helper に置き換える。

**理由**: `whitespace0().zip_right(p).zip_left(whitespace0())` を値・区切り記号・括弧に全部かけると、`[1, 2, 3]` のような短い入力でも空白のための parser 合成が多くなる。subset ベンチは fixed cost を観測する場でもあるため、JSON grammar を維持しつつ責務を絞った構成のほうが目的に合う。

**代替案**:
- subset ベンチは readability 優先で `ws()` を残す
  - 変更範囲は減るが、今回の改善対象から一番分かりやすい固定コストが残る
- subset ベンチを full JSON 実装の簡略版に全面差し替えする
  - 既存の combinator ベンチという性格が薄れ、何を測っているかが曖昧になる

## Risks / Trade-offs

**[受理言語のずれ]** 空白消費位置を動かすと、配列末尾や member 境界で受理ケースが変わる可能性がある  
→ 空白入りの array/object/member サンプルを追加し、compact form と同じ AST になることを確認する

**[ベンチの比較可能性低下]** `json_full.rs` と `alloc_count.rs` が別実装のまま進むと、README の説明と実測値がずれる  
→ 共通モジュール化し、oni-comb 側の JSON 実装を一か所に集約する

**[可読性低下]** `ws()` のような単純ラッパーをやめると、subset ベンチのコードが少し冗長になる  
→ `skip_ws`, `comma`, `colon`, `object_member` など責務が読める helper 名を使い、文法境界ごとに整理する

**[効果が限定的]** すでに大きなボトルネックを潰しているため、改善幅は single-digit % に留まる可能性がある  
→ full JSON と subset の両方を再計測し、README では「効いたか / 効かなかったか」をそのまま記録する

## Migration Plan

1. oni-comb 側 full JSON パーサーを共通化し、`json_full.rs` と `alloc_count.rs` の呼び出しを切り替える
2. 空白責務を boundary-scoped helper に再配置する
3. `workloads/json.rs` の subset ベンチを同じ方針で整理する
4. ベンチと allocation count を再実行し、README を更新する

ロールバックは、共通化前の実装に戻し、`json_full.rs` / `alloc_count.rs` のローカル実装を復元すればよい。

## Open Questions

- subset ベンチで readability のために一部 `ws()` を残すか、完全に helper 分割へ寄せるかは、実装後の見通しで最終判断する
- full JSON の再計測に加えて、空白の多い synthetic input を追加で持つかは任意。まずは既存 workload で差分を見る
