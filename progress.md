## 2025-09-26 21:29 JST
- 作業開始。PLAN.mdと現状コードを確認。

## 2025-09-26 21:39 JST
- ParseResultにflat_map/and_then/rest/state/lengthを追加しAPI拡充を開始。
- Parserにmap/flat_map/filter/attempt/many0/many1/runを実装。

## 2025-09-26 21:49 JST
- Parserのコンビネータ(map/flat_map/filter/attempt/many系)とPrelude関数を実装。
- ParseStateのCopy/Clone実装を調整し、結合時のコミット制御を整備。
- 結合テスト(combinators)とParseResult単体テストを追加し `cargo test` を実行。

## 2025-09-26 22:00 JST
- .gitignoreを追加し.idea/Cargo.lock/targetを無視設定。
- criterion/nom/pomをdev-depsに追加しベンチマーク雛形を作成。

## 2025-09-26 22:10 JST
- CSV形式の整数列を対象にした新規CPUバウンドベンチ(csv_numbers)を追加。
- oni-comb/nom/pomで比較計測を実施 (warm-up 1s, measure 2s)。
- sum_digitsベンチと合わせて `cargo bench --bench compare -- --warm-up-time 1 --measurement-time 2` を確認。

## 2025-09-26 22:31 JST
- take_while1/byte/separated_list1/separated_fold1 を追加し、ホットパス最適化の土台を実装。
- sum_digits ベンチで約350nsまで短縮し、nomとの差は約10## 2025-09-26 22:31 JST
- take_while1/byte/separated_list1/separated_fold1 を追加し、ホットパス最適化の土台を実装。
- sum_digits ベンチで約350nsまで短縮し、nomとの差は約10%まで接近。
- CSVベンチは専用Parser復活＋fold系APIで維持し、nom比1.4〜2倍程度の優位を再確認。
- `cargo bench --bench compare -- --warm-up-time 1 --measurement-time 2` と `cargo test` を再実行。

