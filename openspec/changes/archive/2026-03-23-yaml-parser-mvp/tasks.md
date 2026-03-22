## 1. Workspace and Crate Setup

- [x] 1.1 workspace に `modules/yaml` を追加し、`oni-comb-yaml` crate の `Cargo.toml` と `lib.rs` を作成する
- [x] 1.2 `modules/json` と同様の公開面に合わせて `parser.rs`、`value.rs`、public export を整える

## 2. Declarative Feasibility

- [x] 2.1 block mapping / block sequence / flow mapping / flow sequence / scalar subset の各 grammar slice を、public combinator chain のみで表現できるか小さく検証する
- [x] 2.2 custom `Parser` 実装、`InputStream` wrapper、`parse_next` / `checkpoint/reset` / token stepping 直呼び、戻り値破棄による入力制御を使わない実装方針を保つ
- [x] 2.3 declarative に表現できない requirement が見つかった場合は、実装を止めて不足理由を OpenSpec へフィードバックする

## 3. AST and Public API

- [x] 3.1 `Null`、`Bool`、`Integer`、`String`、`Sequence`、`Mapping` を持つ MVP YAML AST を定義する
- [x] 3.2 `yaml()` / `yaml_value()` と `parse()` / `parse_value()` 相当の public API を実装する
- [x] 3.3 `parse()` が入力全体を消費し、`parse_value()` が value 単体 parser として使える契約を実装する

## 4. Declarative Grammar Implementation

- [x] 4.1 plain / single-quoted / double-quoted string、`null`、`bool`、10 進 integer、block / flow の line comment handling を実装する
- [x] 4.2 block mapping / block sequence を実装し、indentation に基づくネストと scalar-only mapping key 制約を扱えるようにする
- [x] 4.3 flow mapping / flow sequence を実装し、block grammar との相互ネストを扱えるようにする

## 5. Validation and Error Reporting

- [x] 5.1 scalar / collection / nesting / block-flow comment の受け入れテストを追加する
- [x] 5.2 trailing text、unterminated flow collection、indentation mismatch の失敗ケースを追加する
- [x] 5.3 `parse_value()` が closed flow collection のような終端明確な値を prefix parse できることを検証する
- [x] 5.4 downstream YAML parser が line / column / expected / context を含む parse error を返すことを検証する
- [x] 5.5 実装が parser core への YAML 専用 API 追加なし、かつ命令型 fallback なしで成立したことを docs と OpenSpec の観点で再確認する
