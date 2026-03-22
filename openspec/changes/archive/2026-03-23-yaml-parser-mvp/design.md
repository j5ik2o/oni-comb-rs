## Context

`yaml-ready-parser` により、parser core は layout-sensitive grammar を支える公開契約をすでに備えている。特に `modules/parser/tests/yaml_ready_acceptance.rs` では、行頭判定、期待インデント、flow/block 文脈、simple-key gating、plain scalar boundary を、YAML 専用 API を parser core に追加せずに downstream-owned wrapper / helper で表現できることを確認済みである。

一方で、現時点の `yaml-parser` capability は「YAML 実装は別 change で提案する」という gate しか持っておらず、実際の downstream crate、AST、public API、テスト方針はまだ存在しない。この change では readiness の実証を本物の downstream crate に進めるが、対象は full YAML ではなく MVP subset に限定する。

## Goals / Non-Goals

**Goals:**
- `modules/yaml` crate を追加し、`oni-comb-yaml` として最小の downstream YAML parser を提供する
- block mapping / block sequence / flow mapping / flow sequence を single-document grammar として parse できるようにする
- plain / single-quoted / double-quoted string、`null`、`bool`、10 進 integer、comment をサポートする
- parser 全体を関数型・宣言的・public combinator chain のみで記述する
- parser core へ YAML 専用 API を追加せずに実装し、表現不能なら不足をフィードバックできるようにする
- line / column / context を含む `ParseError` ベースの診断を downstream parser でも保つ

**Non-Goals:**
- YAML 1.2 全面対応
- block scalar (`|`, `>`)、chomping、folding の実装
- anchor / alias / merge key / tag / multi-document の実装
- advanced numeric schema（16 進、8 進、指数、`.inf`、`.nan` など）の完全対応
- parser core へ新しい YAML 専用 primitive / combinator / layout API を追加すること
- custom `Parser` 実装、`InputStream` wrapper、`parse_next` / `checkpoint/reset` / `next_token` 直呼び、戻り値破棄による命令型制御で不足を埋めること

## Decisions

### D1. `modules/yaml` は `modules/json` と同様の薄い crate 構成にする

`modules/yaml` は `parser.rs`、`value.rs`、`lib.rs` を中心とした薄い downstream crate とする。public API は `yaml()` / `yaml_value()` と `parse()` / `parse_value()` 相当の 2 層を持たせる。

- Why: 既存 downstream crate の構成に揃えると、公開面とテスト方針が一貫する
- Alternative considered: 初期から細かいモジュールへ分割する
- Why not: MVP 段階ではファイル分割より、公開契約と grammar の成立性の確認を優先したい

### D2. YAML 文脈も declarative な combinator 合成だけで表現を試みる

indentation、flow/block、simple-key のような YAML 文脈も、まずは existing public combinator と parser 出力の合成だけで表現を試みる。`StrInputStream` を包む wrapper や custom `Parser` 実装で stateful adaptation を隠蔽することは、この change では許容しない。

- Why: この change の価値は「downstream 実装まで declarative に書けるか」を検証することにあり、imperative wrapper に逃げると不足の有無が見えなくなる
- Alternative considered: `yaml_ready_acceptance` と同様に wrapper / helper parser を production 実装へ持ち込む
- Why not: parser core の readiness を downstream 実装で再検証する目的がぼやける

### D3. helper も pure helper function に限定し、命令型 escape hatch を禁止する

top-level だけでなく内部抽象も public combinator chain の組み合わせで構成する。許容する helper は「parser を返す関数」までとし、custom `Parser` 実装、`parse_next` / checkpoint / reset / token stepping の直接利用、戻り値破棄による input 消費は行わない。

- Why: helper の内部だけ命令型にすると、実質的に parser combinator を使わない実装でも通ってしまう
- Alternative considered: top-level だけ declarative に見せて、helper の中で imperative に処理する
- Why not: 今回まさにその方向が方針違反だったため

### D4. AST は最小 enum とし、mapping は順序保持を優先する

MVP の AST は `Null`、`Bool`、`Integer`、`String`、`Sequence`、`Mapping` を持つ最小 enum とする。`Mapping` は `Vec<(YamlValue<'a>, YamlValue<'a>)>` か同等の順序保持表現を使い、重複キーの解釈は MVP では固定しない。ただし grammar として受理する key は supported scalar subset に限定し、explicit key (`? key`) や collection-valued key はこの change では扱わない。

- Why: YAML では key order や duplicate key policy が後で論点になりやすく、`BTreeMap` へ早期固定すると表現力を狭める
- Alternative considered: `BTreeMap` を使って JSON と同型にする
- Why not: duplicate key の扱いを勝手に消してしまい、後続 change の余地を減らす

### D5. scalar subset は basic schema に限定する

MVP scalar は plain / single-quoted / double-quoted string、`null`、`true` / `false`、10 進 integer に限定する。plain scalar は block と flow で停止条件を変える。comment は quoted scalar の内部では解釈せず、quoted scalar 外では value boundary の後に現れる `#` から行末までを line comment として無視する。

- Why: YAML らしい文脈依存を保ちつつ、block scalar や advanced numeric schema まで広げないことで MVP を保てる
- Alternative considered: 既存 `yaml-parser` spec にあった full scalar set を最初から実装する
- Why not: layout-sensitive parser の成立検証よりも仕様消化の比重が大きくなりすぎる

### D6. single document + full-consume parse を MVP の public contract にする

`parse()` は 1 つの top-level YAML value を parse し、入力末尾まで消費する。`parse_value()` は value 単体 parser として EOF を要求せず、closed flow collection や quoted scalar のように終端が明確な値を prefix parse できる契約とする。document start / end marker や multi-document はこの change では扱わない。

- Why: JSON crate と同じ API 形に揃えつつ、multi-document の複雑さを後続 change に送れる
- Alternative considered: 最初から `Vec<Document>` を返す
- Why not: top-level contract と AST が一気に重くなり、MVP を超える

### D7. 表現不能なら実装を止めて不足を報告する

ある grammar slice が public combinator chain だけでは表現不能だと判明した場合、その時点で実装を止めて不足理由をフィードバックする。報告では「MVP scope が広すぎる」のか、「parser core の generic capability が足りない」のか、「spec が曖昧なのか」を切り分ける。

- Why: imperative fallback で埋めると、本当に不足しているものが見えなくなる
- Alternative considered: 一時的な命令型コードで unblock して先に進む
- Why not: feedback loop を失い、この change の検証価値が下がる

## Risks / Trade-offs

- [Risk] plain scalar の停止条件が曖昧だと flow/block の境界で誤受理しやすい → Mitigation: `yaml_ready_acceptance` の flow plain scalar litmus を直接参照し、flow と block の停止条件を acceptance test で固定する
- [Risk] declarative-only 制約の下では一部 grammar が書けない可能性がある → Mitigation: その場合は implementation を停止し、不足する generic capability か scope 問題として artifacts にフィードバックする
- [Risk] duplicate key policy を未決定のまま進めると後で互換性論点になる → Mitigation: MVP では mapping representation を順序保持にして policy を固定しない
- [Risk] block scalar を除外すると「YAML らしさが足りない」と見える → Mitigation: proposal と spec で block scalar を明示的に non-goal とし、後続 change に切り出す

## Migration Plan

1. `modules/yaml` crate を workspace に追加する
2. MVP AST と public parse API を追加する
3. 各 grammar slice を public combinator chain だけで表現できるか小さく検証する
4. block / flow collection と scalar subset を declarative に実装する
5. parser / AST / error / comment handling の acceptance tests を追加する
6. 表現不能な箇所が見つかった場合は、scope / capability / spec のどこに不足があるかを artifacts へ反映する
7. full YAML scope へ拡張したくなった場合は、block scalar や anchors などを別 change として提案する

## Open Questions

- single-quoted / double-quoted string でどこまで escape を許すか
