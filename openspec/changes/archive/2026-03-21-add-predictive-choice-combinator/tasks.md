## 1. API and Runtime Design

- [x] 1.1 predictive choice の first cut scope を `StrInputStream` / `ByteInputStream` の先頭 byte dispatch に固定する
- [x] 1.2 public API 名と利用形 (`predictive_choice`, `dispatch_by_byte`, builder 形式など) を決め、 combinator-chain を維持できる形にする
- [x] 1.3 unmatched Backtrack、選択後 non-fallback、入力非消費の意味論をテスト可能な形で固定する

## 2. Combinator Implementation

- [x] 2.1 predictive choice combinator 型と必要な public facade を追加する
- [x] 2.2 exact-byte branch と軽い predicate branch を扱える first cut を実装する
- [x] 2.3 `or` 連鎖より少ない trial/reset で branch dispatch できるようにする

## 3. Validation

- [x] 3.1 branch selection、unmatched Backtrack、selected-branch failure propagation の unit test を追加する
- [x] 3.2 representative grammar として benchmark 用 JSON parser へ適用し、受理集合が変わらないことを確認する
- [x] 3.3 必要なら `modules/json` など downstream parser でも API の使い勝手を確認する

## 4. Benchmark and Documentation

- [x] 4.1 `comparison -- json` または `json_full` で before/after を比較し、効果を測定する
- [x] 4.2 README / benches README に predictive choice の用途、効果、非 goal を記録する
