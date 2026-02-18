```markdown
# レガシーコード削除レビュー

## 結果: APPROVE / REJECT

## サマリー
{1-2文で結果を要約}

## 確認した観点
- [x] 削除対象の妥当性
- [x] 削除漏れ
- [x] ビルド・テスト通過
- [x] 公開API互換性
- [x] 削除範囲の適切性

## 今回の指摘（new）
| # | finding_id | 場所 | 問題 | 修正案 |
|---|------------|------|------|--------|
| 1 | REM-NEW-src-file-L42 | `src/file.rs:42` | 問題の説明 | 修正方法 |

## 継続指摘（persists）
| # | finding_id | 前回根拠 | 今回根拠 | 問題 | 修正案 |
|---|------------|----------|----------|------|--------|
| 1 | REM-PERSIST-src-file-L77 | `src/file.rs:77` | `src/file.rs:77` | 未解消 | 既存修正方針を適用 |

## 解消済み（resolved）
| finding_id | 解消根拠 |
|------------|----------|
| REM-RESOLVED-src-file-L10 | 削除済み |

## REJECT判定条件
- `new` または `persists` が1件以上ある場合のみ REJECT 可
- `finding_id` なしの指摘は無効
```

**認知負荷軽減ルール:**
- APPROVE → サマリーのみ（5行以内）
- REJECT → 該当指摘のみ表で記載（30行以内）
