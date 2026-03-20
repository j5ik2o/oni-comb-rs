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

parser core の基盤コンビネータ実装を除き、top-level の下流 grammar 記述では `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、入力状態を読んだ手書き if/else 分岐を禁止する。grammar 記述は、まず public combinator のメソッドチェインと下流所有 helper parser の組み合わせで完結することを要求する。

ここでいう helper parser は、既存の公開契約（`InputStream` / `Checkpoint`、位置情報 API、error model、既存 combinator）を組み合わせて downstream 側の stateful adaptation をカプセル化する小さな補助実装を指す。helper の内部で `Parser` / `InputStream` を直接扱うことは許容するが、その命令型処理を top-level grammar 定義へ漏らしてはならない。`fn_parser` は parser capability の不足を補う escape hatch としては使わず、同値な宣言的実装が先に存在する箇所の局所最適化に限って使う。

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

### D4. YAML 必要機能は既存の公開契約と downstream 側の合成でまず実証する

YAML パーサー実装で必要な機能を、そのまま parser core の新 API として搭載する前に、既存の公開契約でどこまで表現できるかをまず実証する。ここでいう公開契約には `InputStream` / `Checkpoint`、位置情報 API、error model、既存 combinator、および下流クレートが独自 `InputStream` を実装して state を所有できる拡張点を含む。

`YAML-ready` 判定に必要なのは YAML 専用 Layout API の有無ではなく、下流 grammar が parser モジュールの既存公開契約を組み合わせて必要な振る舞いを記述できることである。litmus grammar で表現不能なケースが再現した場合にのみ、その不足を YAML 非依存の最小 generic capability として抽出する。

**分解マッピング（現時点の実証形）:**

| YAML 必要機能 | まず使う公開契約 | 用途例 |
|--------------|------------------|--------|
| flow/block nesting | checkpoint 可能な下流所有 state + 既存 combinator | YAML, JSON, 任意の括弧文脈 |
| 期待インデント | `line_start` / `column` / checkpoint 可能な下流所有 state | YAML, Python, Haskell, Makefile |
| simple_key_allowed | checkpoint 可能な下流所有 state + `or` / `attempt` | 任意の scoped boolean state |
| 行頭判定 | `offset` / `line_start` などの位置情報 API | 全 layout-sensitive grammar |
| 位置付き診断 | `ParseError`, `ExpectError`, context 付与 | 全 layout-sensitive grammar |

**代替案:**
- YAML 特化の flow_level, simple_key_allowed 等を直接 core に実装する
- 不足の再現を待たず、仮説ベースで generic API を先回り追加する

**理由:**
- parser core は YAML を知らず、YAML は core の既存公開契約と downstream 側 helper を組み合わせるだけで実装できることを先に示すべきである
- 同じ土台で Python, Haskell, Makefile 等の layout-sensitive grammar も書ける
- litmus grammar が既存契約で成立している段階では、新 API の追加は YAGNI になりやすい

### D5. 新しい layout-aware primitive / combinator は不足が再現した場合に限り追加を検討する

この change では、layout-aware な振る舞いを parser core の新しい public API として先に固定しない。まずは downstream 側の合成で litmus grammar が成立するかを受け入れ条件とし、既存公開契約だけでは表現不能なケースが残ったときに限り、YAML 非依存の最小 generic primitive / combinator を検討する。

将来 generic 追加を検討する場合でも、候補は `at_line_start()`, `at_indent(min)`, `with_flag(...)` のような汎用名に限り、YAML 固有語彙を parser core に持ち込まない。

**代替案:**
- litmus grammar の結果にかかわらず、layout-aware combinator を先に public API 化する
- すべての layout-aware ロジックを下流クレート専用 helper に閉じ込め、parser core 側の不足を永続的に放置する

**理由:**
- 先行 API 追加は、今回の「Layout API は作らない」という方針と衝突する
- 一方で本当に表現不能なケースが出た場合は、最小 generic 追加で parser core の責務不足を補える余地は残しておくべきである
- `modules/parser/tests/yaml_ready_acceptance.rs` の litmus grammar は、現時点では新しい Layout API なしで、top-level grammar を declarative に保ったまま成立している

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
3. 行頭判定、期待インデント判定、flow/block 文脈判定、boolean flag 判定を、既存公開契約と downstream 側の合成でどう実現するかを設計に反映する
4. litmus grammar で表現不能なケースが残る場合に限り、YAML 非依存の最小 generic primitive / combinator の追加要否を評価する
5. 追加が必要な場合にのみ、parser モジュールの最小実装タスクへ進む
6. litmus grammar が既存契約または必要最小限の generic 追加で記述できた段階で、YAML クレート着手の可否を再判定する

## Open Questions

- 将来 generic 追加が必要になった場合、state carrier を `InputStream` トレイトの associated type にするか、下流側 helper / wrapper に留めるか
- layout state のデフォルト値を持たせるか、grammar クレート側で初期化を強制するか
- `position()` や span 抽出 API をどこまで public contract に含めるか

## Readiness Result

- `modules/parser/tests/yaml_ready_acceptance.rs` の litmus grammar 群を parser モジュール単体で通過したため、この change の `YAML-ready` 判定は YAML 実装着手ではなく acceptance test により充足された
- 実証は YAML 専用の Layout API を `modules/parser` に導入せず、checkpoint 可能な state、位置情報、エラーモデル、既存 combinator、および downstream 側 helper の組み合わせで行っている
- downstream YAML 実装は、この change で整えた parser/core capability を組み合わせる前提で着手できる
- この change では litmus grammar 実装に `fn_parser` を導入していないため、`fn_parser` は capability 実現手段として使われていない。将来導入する場合でも、宣言的実装が先に存在し、性能根拠が確認できる場合に限る
