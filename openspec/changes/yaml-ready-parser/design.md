## Context

現在の oni-comb-parser は、文字列やバイト列を直列に消費する parser combinator としては十分な機能を持つが、YAML 1.2 のような layout-sensitive grammar を combinator だけで自然に記述できる保証はない。特に backtrack の対象が入力位置に偏っており、インデントや flow/block 文脈のような layout state を安全に巻き戻せない。結果として、将来 YAML 実装を始めたときに下流クレート側で `parse_next`、`checkpoint/reset`、戻り値破棄、外部可変状態を多用して parser core の不足を埋める設計破綻が起きるリスクがある。

この変更では YAML クレートはまだ作らず、parser モジュール単体に対して `YAML-ready` の契約を定義する。焦点は「YAML を実装できるか」ではなく、「YAML 実装に必要な能力を parser が単体で提供できるか」である。

## Goals / Non-Goals

**Goals:**
- YAML 実装に必要な parser capability を、parser モジュール単体の acceptance criteria として明文化する
- 下流 grammar 実装における命令型 escape hatch を禁止し、public combinator のメソッドチェインだけで記述できることを契約化する
- checkpoint 対象に含めるべき state と、後段で扱う semantic state を分離する
- 位置情報モデルとエラーモデルを、layout-sensitive grammar に耐える形に再定義する
- 実装前に litmus grammar 群を定義し、parser redesign の成否を検証できるようにする

**Non-Goals:**
- `modules/yaml` の新規実装
- YAML 1.2 の全仕様の詳細確定
- parser モジュールの実装をこの変更内で完了させること
- JSON / URI / crond など既存下流クレートの機能追加

## Decisions

### D1. `YAML-ready` は YAML 実装ではなく litmus grammar で判定する

`YAML-ready` は「YAML クレートを書き始めて詰まらなかった」ではなく、「YAML に必要な grammar を parser モジュール単体の public combinator で記述できる」こととして判定する。試金石は block list、indent nesting、flow/block 切替、multiline block、document boundary、indent error などの litmus grammar 群とする。

**代替案:**
- 先に YAML クレートを作り、途中で不足 API を補う
- parser の readiness を informal なチェックリストだけで運用する

**理由:**
- YAML 実装着手後に parser core の責務不足が判明するのを防げる
- litmus grammar は parser 単体の test contract に落としやすい

### D2. 下流 grammar 実装では命令型 escape hatch を禁止する

parser core の基盤コンビネータ実装を除き、下流 grammar 実装では `parse_next` 直呼び、`checkpoint/reset` 直呼び、戻り値破棄、入力状態を読んだ手書き if/else 分岐を禁止する。grammar 記述は public combinator のメソッドチェインだけで完結することを要求する。

**代替案:**
- YAML クレートだけ特例で `parse_next` を許可する
- public combinator と命令型 parser を混在させる

**理由:**
- 下流で `parse_next` を濫用し始めた時点で parser combinator としての抽象化が破綻している
- 禁止事項を acceptance criteria に入れることで、設計破綻を早期に検知できる

### D3. state を `input state`、`checkpointable layout state`、`semantic state` に分離する

parser が直接扱う state を 3 層に分ける。

- `input state`: offset / line / column / line anchor / span のような入力位置情報
- `checkpointable layout state`: indent stack、flow level、simple-key 許可状態、block-in / block-out 相当の文脈
- `semantic state`: anchors、aliases、tag resolution のような後段処理や AST 構築で扱える状態

backtrack の対象に含めるのは `input state` と `checkpointable layout state` までとし、`semantic state` は別設計とする。

**代替案:**
- すべての YAML 関連 state を checkpoint 対象に含める
- input state だけを checkpoint し、layout state は下流クレートで手動巻き戻しする

**理由:**
- anchors などの大きい状態まで checkpoint に含めるとコストと複雑さが増える
- 一方で layout state を checkpoint 対象から外すと `or` / `attempt` で文脈破損が起きる

### D4. layout-sensitive grammar 用の専用 capability を parser core に昇格する

YAML 実装に必要な観測・遷移を、generic な `guard(Fn(&I) -> bool)` だけに押し込めず、layout-aware primitive / combinator として parser core の public API に昇格させる。対象には「行頭判定」「期待インデント判定」「flow/block 文脈判定」「layout state を伴う checkpoint/reset」が含まれる。

**代替案:**
- `guard` だけで表現し、複雑な述語を下流に書かせる
- YAML クレート側にヘルパー関数を置いて実質的に命令型で組む

**理由:**
- `guard` は観測しかできず、layout state の遷移や巻き戻しを表現できない
- capability を core に持ち上げることで、YAML 以外の layout-sensitive grammar にも再利用可能になる

### D5. error model は生成時点で location/context を持つ

`ExpectError` は position だけを受け取る形ではなく、入力と layout context の現在値から location/context を構築できる方向へ見直す。`fill_location_from_src()` のような後付け全走査は公開 API の最終手段に留め、parser の主要経路では使わない。

**代替案:**
- 現行の `position: usize` ベースを維持し、公開 API で後付け計算する
- location を `position()` combinator の戻り値にだけ頼る

**理由:**
- layout-sensitive grammar では「どの文脈で何を期待したか」が診断に重要で、後付け走査では不十分
- error generation が入力状態に結びついていないと、設計上の責務が曖昧なまま残る

### D6. `line_start` は列番号ではなく行アンカーとして扱う

`line_start` は column と同じ単位に揃えることを目的にせず、「現在行の先頭を指すアンカー」であることを責務として明示する。`column` は人間向けの列番号、`line_start` は行スライスや span 抽出のための anchor として分離して扱う。

**代替案:**
- `line_start` を char 単位に変換する
- `line_start` を廃止する

**理由:**
- `&str` / `&[u8]` の slice は byte offset で扱うほうが自然
- 問題の本質は単位混在そのものではなく、責務が曖昧なまま `Checkpoint` に同居している点にある

## Risks / Trade-offs

- [Risk] 受け入れ条件が強すぎて parser redesign のスコープが大きくなる → Mitigation: YAML 実装ではなく litmus grammar を先に固定し、必要 capability だけを抽出する
- [Risk] layout state を checkpoint 対象に入れると performance regression が起きる → Mitigation: semantic state は対象外にし、checkpoint は軽量な Copy state に限定する
- [Risk] 既存 API との後方互換性が壊れる → Mitigation: breaking points を spec で明示し、既存 capability ごとに変更を局所化する
- [Risk] `parse_next` 禁止を曖昧に運用すると形骸化する → Mitigation: tests と tasks に「下流 grammar 実装での禁止事項」を明示し、litmus grammar を review gate にする
- [Risk] `line_start` 問題だけに議論が引っ張られ、本質である layout context が置き去りになる → Mitigation: proposal と specs の中心を `checkpointable layout context` に置く

## Migration Plan

1. `YAML-ready` の acceptance criteria と litmus grammar を spec として固定する
2. 既存の位置情報 spec と error spec を delta spec で更新する
3. parser core が提供すべき state model と public capability を設計で確定する
4. その後に初めて parser モジュールの実装タスクへ進む
5. litmus grammar が public combinator だけで記述できた段階で、YAML クレート着手の可否を再判定する

## Open Questions

- layout state を `InputStream` に統合するか、別の parser state abstraction に切り出すか
- layout-aware primitive を generic API にするか、text input 系に限定した API にするか
- semantic state のうち anchors / aliases を parser core の責務に含めるか、YAML クレート側に残すか
- `position()` や span 抽出 API をどこまで public contract に含めるか
