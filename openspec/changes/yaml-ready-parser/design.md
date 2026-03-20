## Context

現在の oni-comb-parser は、文字列やバイト列を直列に消費する parser combinator としては十分な機能を持つが、YAML 1.2 のような layout-sensitive grammar を combinator だけで自然に記述できる保証はない。特に backtrack の対象が入力位置に偏っており、インデントや flow/block 文脈のような layout state を安全に巻き戻せない。結果として、将来 YAML 実装を始めたときに下流クレート側で `parse_next`、`checkpoint/reset`、戻り値破棄、外部可変状態、`fn_parser` を多用して parser core の不足を埋める設計破綻が起きるリスクがある。

この変更では YAML クレートはまだ作らず、parser モジュール単体に対して `YAML-ready` の契約を定義する。焦点は「YAML を実装できるか」ではなく、「YAML 実装に必要な能力を parser が単体で提供できるか」である。

## Goals / Non-Goals

**Goals:**
- YAML 実装に必要な parser capability を、parser モジュール単体の acceptance criteria として明文化する
- 下流 grammar 実装における命令型 escape hatch を原則禁止し、public combinator のメソッドチェインだけで記述できることを契約化する
- `fn_parser` を capability 不足の代替手段ではなく、宣言的実装成立後の局所最適化手段として位置づける
- checkpoint 対象に含めるべき parser core state と、parse 後に扱う downstream semantic data を分離する
- 位置情報モデルとエラーモデルを、layout-sensitive grammar に耐える形に再定義する
- 実装前に litmus grammar 群を定義し、parser redesign の成否を検証できるようにする

**Non-Goals:**
- `modules/yaml` の新規実装
- YAML 1.2 の全仕様の詳細確定
- `YAML-ready` 判定に直接必要ない parser モジュール全面刷新
- JSON / URI / crond など既存下流クレートの機能追加

## Decisions

### D1. `YAML-ready` は YAML 実装ではなく litmus grammar で判定する

`YAML-ready` は「YAML クレートを書き始めて詰まらなかった」ではなく、「YAML に必要な grammar を parser モジュール単体の public combinator で記述できる」こととして判定する。試金石は block list、indent nesting、flow/block 切替、multiline block、block scalar header、document boundary、simple-key gating、simple-key backtrack、flow plain scalar boundary、indent error などの litmus grammar 群とする。

**代替案:**
- 先に YAML クレートを作り、途中で不足 API を補う
- parser の readiness を informal なチェックリストだけで運用する

**理由:**
- YAML 実装着手後に parser core の責務不足が判明するのを防げる
- litmus grammar は parser 単体の test contract に落としやすい
- simple-key rollback、flow scalar 停止条件、block scalar header は命令型 escape hatch に落ちやすいので、受け入れ条件で先に検証する価値が高い

### D2. 下流 grammar 実装では命令型 escape hatch を原則禁止し、`fn_parser` は最適化用途に限定する

parser core の基盤コンビネータ実装を除き、下流 grammar 実装では `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、入力状態を読んだ手書き if/else 分岐を禁止する。grammar 記述は、まず public combinator のメソッドチェインだけで完結することを要求する。`fn_parser` は parser capability の不足を補う escape hatch としては使わず、同値な宣言的実装が先に存在する箇所の局所最適化に限って使う。

**代替案:**
- YAML クレートだけ特例で `parse_next` を許可する
- public combinator と命令型 parser を混在させる
- 最初から `fn_parser` を一般的な実装手段として許可する

**理由:**
- 下流で `parse_next` を濫用し始めた時点で parser combinator としての抽象化が破綻している
- `fn_parser` を一般解として許すと parser capability の不足が隠蔽され、`YAML-ready` 判定が空洞化する
- 一方で局所最適化の余地は残したいので、宣言的実装成立後の optimization escape hatch としてのみ許可する

### D3. parser core の state と parse 後に扱う semantic data を分離する

parser core が checkpoint model として直接扱う state は 2 層に分ける。

- `input state`: offset / line / column / line anchor / span のような入力位置情報
- `checkpointable layout state`: インデントスタック、コンテキストスタックのような汎用的な layout 構造

これとは別に、YAML 下流実装が parse 成功後に扱う `downstream semantic data` を分離する。anchors、aliases、tag resolution は parser core の checkpoint 対象 state として持つことを前提にせず、まずは YAML 構文要素としてパースし、その後に resolver / AST 構築側で解決する。

backtrack の対象に含めるのは `input state` と `checkpointable layout state` までとする。anchors / aliases / tags のような YAML 固有の意味解釈は、parser core の checkpoint model とは切り離して設計する。

**代替案:**
- すべての grammar 関連 state を checkpoint 対象に含める
- input state だけを checkpoint し、layout state は下流クレートで手動巻き戻しする

**理由:**
- anchors や aliases を parser core の checkpoint 対象 state に含めるとコストと複雑さが増える
- 一方で layout state を checkpoint 対象から外すと `or` / `attempt` で文脈破損が起きる
- YAML 固有の意味解釈を parse 後フェーズへ送ることで、parser core は汎用 capability に集中できる

### D4. YAML 必要機能を汎用機能に分解して parser core に提供する

YAML パーサー実装で必要な機能をそのまま parser core に搭載するのではなく、汎用的な機能に分解して提供する。各 grammar クレートは汎用コンビネータを組み合わせて必要な機能を実現する。

**分解マッピング:**

| YAML 必要機能 | 汎用機能 | 用途例 |
|--------------|---------|--------|
| flow_level (flow/block nesting) | ContextStack<T> | YAML: Flow/Block, JSON: Object/Array, 任意: 括弧種別 |
| indent_stack (期待インデント) | IndentStack | YAML, Python, Haskell, Makefile |
| simple_key_allowed | FlagSet<F> | YAML: `SimpleKeyAllowed`, 任意: scoped boolean state |
| 行頭判定 | Position Query | 全 layout-sensitive grammar |
| インデント判定 | at_indent(n) | 全インデントベース言語 |

**代替案:**
- YAML 特化の flow_level, simple_key_allowed 等を直接 core に実装する
- guard(Fn) だけで表現し、複雑な述語を下流に書かせる

**理由:**
- parser core は YAML を知らず、YAML は core の汎用機能を組み合わせるだけで実装できる
- 同じ core で Python, Haskell, Makefile 等の layout-sensitive grammar も書ける
- guard は観測しかできず、flag を含む layout state の遷移や巻き戻しを表現できない

### D5. layout-aware 汎用コンビネータを提供する

汎用機能を使うコンビネータを parser core に提供する。

**データ構造:**
- `IndentStack`: 固定長インデントスタック（Copy, Default）
- `ContextStack<T>`: 汎用コンテキストスタック（Copy, Default）
- `FlagSet<F>`: 任意の boolean flag を保持する checkpoint 可能な集合

**コンビネータ:**
- `at_line_start()`: 行頭でのみ成功
- `at_indent(min)`: 現在列 >= min なら成功
- `push_indent(indent, parser)`: インデントを push して parser 実行、終了時に pop
- `in_context(ctx, parser)`: コンテキストに入って parser 実行、終了時に exit
- `flag_is(flag, value)`: 指定 flag が現在値 `value` のとき成功
- `with_flag(flag, value, parser)`: 指定 flag を一時的に `value` に設定して parser 実行

**代替案:**
- 各 grammar クレートで独自にコンビネータを実装する
- 低レベル API だけ提供し、コンビネータは提供しない

**理由:**
- コンビネータレベルで提供することで、下流 grammar 実装での命令型コードを防げる
- generic flag capability を first-class にすることで、`simple_key_allowed` のような条件付き文脈を宣言的に扱える
- Checkpoint との連携（自動巻き戻し）を正しく実装できる

### D6. error model は生成時点で location/context を持つ

`ExpectError` は position だけを受け取る形ではなく、入力と layout context の現在値から location/context を構築できる方向へ見直す。`fill_location_from_src()` のような後付け全走査は公開 API の最終手段に留め、parser の主要経路では使わない。

**代替案:**
- 現行の `position: usize` ベースを維持し、公開 API で後付け計算する
- location を `position()` combinator の戻り値にだけ頼る

**理由:**
- layout-sensitive grammar では「どの文脈で何を期待したか」が診断に重要で、後付け走査では不十分
- error generation が入力状態に結びついていないと、設計上の責務が曖昧なまま残る

### D7. `line_start` は列番号ではなく行アンカーとして扱う

`line_start` は column と同じ単位に揃えることを目的にせず、「現在行の先頭を指すアンカー」であることを責務として明示する。`column` は人間向けの列番号、`line_start` は行スライスや span 抽出のための anchor として分離して扱う。

**代替案:**
- `line_start` を char 単位に変換する
- `line_start` を廃止する

**理由:**
- `&str` / `&[u8]` の slice は byte offset で扱うほうが自然
- 問題の本質は単位混在そのものではなく、責務が曖昧なまま `Checkpoint` に同居している点にある

## Risks / Trade-offs

- [Risk] 受け入れ条件が強すぎて parser redesign のスコープが大きくなる → Mitigation: YAML 実装ではなく litmus grammar を先に固定し、必要 capability だけを抽出する
- [Risk] layout state を checkpoint 対象に入れると performance regression が起きる → Mitigation: downstream semantic data は対象外にし、checkpoint は軽量な Copy state に限定する（IndentStack, ContextStack, FlagSet は固定長で Copy）
- [Risk] 既存 API との後方互換性が壊れる → Mitigation: breaking points を spec で明示し、既存 capability ごとに変更を局所化する
- [Risk] `parse_next` / `fn_parser` の禁止を曖昧に運用すると形骸化する → Mitigation: tests と tasks に「下流 grammar 実装での禁止事項」と `fn_parser` の許可条件を明示し、litmus grammar を review gate にする
- [Risk] 汎用機能の抽象レベルが不適切で、YAML 以外で使いにくい → Mitigation: litmus grammar として Python/Makefile 風の例も検証対象に含める

## Migration Plan

1. `YAML-ready` の acceptance criteria と litmus grammar を spec として固定する
2. 既存の位置情報 spec と error spec を delta spec で更新する
3. 汎用 layout 機能（IndentStack, ContextStack, FlagSet, Position Query）の API を設計で確定する
4. 汎用コンビネータ（at_line_start, at_indent, push_indent, in_context, flag_is, with_flag）の API を設計で確定する
5. その後に初めて parser モジュールの実装タスクへ進む
6. litmus grammar が汎用コンビネータだけで記述できた段階で、YAML クレート着手の可否を再判定する

## Open Questions

- `IndentStack`, `ContextStack<T>` を `InputStream` トレイトの associated type にするか、具象型として提供するか
- layout state のデフォルト値を持たせるか、grammar クレート側で初期化を強制するか
- `position()` や span 抽出 API をどこまで public contract に含めるか
