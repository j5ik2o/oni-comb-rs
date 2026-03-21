## Context

`recursive()` の owner/ref 分離で再帰の税金は少し下がったが、`json_full` は依然として `or` 連鎖による branch dispatch と checkpoint/reset の影響を強く受けている。特に JSON value のような grammar は先頭 byte で候補をかなり強く絞れるのに、現状の `.or()` は左から順に parser を実行して Backtrack を見てから巻き戻すため、予測可能な choice に対して余分な仕事が多い。

一方で、このプロジェクトでは downstream parser を public combinator chain で書く方針が強く、`fn_parser` や manual `parse_next` で先頭 byte dispatch を書く方向は採らない。したがって、改善は「手続き化」ではなく、同じ declarative style のまま branch dispatch を軽くする public combinator として提供するのが筋である。

## Goals / Non-Goals

**Goals:**
- 先頭 byte に基づいて候補 parser を絞る predictive choice を public API として追加する
- `StrInputStream` と `ByteInputStream` で non-consuming な先頭 byte 観測を使い、通常の `or` 連鎖より少ない checkpoint/reset で分岐できるようにする
- JSON の value choice のような grammar を public combinator chain のままより安く書けるようにする
- 選択後の parser の Backtrack/Cut semantics は既存 contract と整合させる
- first cut では実装コストを抑え、最大の効果が出る先頭 byte dispatch に scope を絞る

**Non-Goals:**
- 任意の `InputStream` に対する完全に汎用な predictive dispatch framework を first cut で作ること
- parser graph を imperative に書き換えること
- `or` を置き換える汎用最適化を一度に全部入れること
- char/token 全般に対する高度な trie compiler や DFA builder を導入すること
- JSON / YAML の full parser rewrite をこの change で完了すること

## Decisions

### D1. first cut は `peek_byte` ベースに限定する

predictive choice の first cut は `StrInputStream::peek_byte()` と `ByteInputStream::peek_byte()` を前提にする。ASCII 主体の grammar で最大の効果が見込め、既存 JSON benchmark と相性がよいからである。

- Why: いま最も大きい候補は JSON のような先頭 byte で分岐が決まる grammar であり、ここに絞るのが最もコスパがよい
- Alternative considered: 任意 token を扱う fully generic selector API
- Why not: trait 制約と API 設計が重くなり、first cut の価値が薄まる

### D2. API は public combinator/builder として提供し、manual dispatch を要求しない

利用者は `predictive_choice` 相当の builder で `when_byte(...)` / `otherwise(...)` のように branch を宣言し、最終的に 1 つの `Parser` を得る形にする。利用者に `match input.peek_byte()` や `parse_next` の直接呼び出しを要求しない。

- Why: この change の価値は declarative style を維持したまま choice の税金を下げることにある
- Alternative considered: `guard(peek_byte == ...)` と `or` の組み合わせで代用する
- Why not: call site が冗長になり、checkpoint/reset 連鎖も依然として残る

### D3. predictive choice は「先に branch を選び、その後は fallback しない」意味論にする

selector で branch が選ばれた後は、その branch parser を実行し、成功なら成功、Backtrack/Cut ならそのまま返す。他 branch への fallback はしない。selector でどの branch に入るかを決められなかった場合だけ、unmatched Backtrack を返す。

- Why: predictive choice の主目的は「間違った branch を順に試すコスト」を消すことであり、選択後に fallback を許すと `or` に戻ってしまう
- Alternative considered: 選択 branch が Backtrack したら次候補へ進む
- Why not: semantics が曖昧になり、最悪ケースでまた linear trial に戻る

### D4. branch 条件は byte equality と軽い predicate を許可する

first cut では単一 byte 一致と `Fn(u8) -> bool` 相当の軽い predicate による branch 指定を許可する。これにより `b'n'`, `b't'`, `b'f'`, `b'"'` による JSON dispatch と、digit-or-minus のような数値 branch を扱えるようにする。

- Why: JSON value dispatch には equality だけでなく `'-' || is_ascii_digit()` のような条件も必要
- Alternative considered: equality only
- Why not: 数値 branch を結局別 combinator で包む必要が出て不便

### D5. 適用先は benchmark / downstream parser の representative grammar に限定して効果測定する

検証はまず benchmark 用 JSON parser と、必要なら `modules/json` の value choice のような representative grammar に限定して行う。全 parser を一度に書き換えず、効果がある場所だけに絞って before/after を取る。

- Why: combinator 追加の ROI を短く確認するには、恩恵の大きい箇所へ限定適用するのがよい
- Alternative considered: JSON/YAML 全面への一括適用
- Why not: diff が大きくなり、効果の原因がぼける

## Risks / Trade-offs

- [Risk] predictive choice の selector 条件が不完全だと、本来受理すべき入力が unmatched Backtrack になる → Mitigation: JSON のような representative grammar で受理集合の回帰テストを追加する
- [Risk] heterogenous branch を builder で持つ設計が型的に複雑になる → Mitigation: first cut は branch 数や API surface を抑え、最も効く byte-based path に限定する
- [Risk] selector 自体のコストが高いと `or` 比で利益が薄い → Mitigation: selector は `peek_byte` + 軽い predicate に限定し、per-branch checkpoint/reset より安いことを benchmark で確認する
- [Risk] selected branch の Backtrack を fallback しない意味論が直感とずれる → Mitigation: `or` と別 combinatorであることを明示し、 spec で deterministic dispatch として定義する

## Migration Plan

1. predictive choice capability を spec として定義する
2. `StrInputStream` / `ByteInputStream` 向け first cut API を設計する
3. combinator 実装と tests を追加する
4. benchmark 用 JSON parser に適用して before/after を確認する
5. 効果が十分なら `modules/json` など他の代表 grammar への適用を別 task で広げる

## Open Questions

- public API 名を `predictive_choice` にするか、`dispatch_by_byte` のようにより具体名にするか
- first cut で `otherwise` branch を必須にするか、unmatched Backtrack を標準にするか
- predicate branch を closure で持つか、common cases 向け helper (`when_byte`, `when_digit`, `when_ascii`) を用意するか
