# ai-review-standalone — AIレビュー（standalone）instruction テンプレート

> **用途**: AI生成コードの専門レビュー（独立ムーブメントとして実行、iteration tracking 付き）
> **使用エージェント**: ai-antipattern-reviewer
> **parallel sub-step 用は `review.md` のバリエーションBを使用**

---

## テンプレート

```
**これは {movement_iteration} 回目のAI Reviewです。**

初回は網羅的にレビューし、指摘すべき問題をすべて出し切ってください。
2回目以降は、前回REJECTした項目が修正されたかの確認を優先してください。

AI特有の問題についてコードをレビューしてください:
- 仮定の検証
- もっともらしいが間違っているパターン
- 既存コードベースとの適合性
- スコープクリープの検出
```

---

## parallel sub-step との違い

| | standalone | parallel sub-step |
|--|-----------|-------------------|
| iteration tracking | あり（`{movement_iteration}`） | なし |
| 初回/2回目の指示分岐 | あり | なし |
| 次のムーブメント | ai_fix or reviewers | 親ムーブメントが決定 |

standalone は ai_review → ai_fix のループを形成するピース向け。
parallel sub-step は review.md のバリエーションBを使う。

---

## 典型的な rules

```yaml
rules:
  - condition: AI特有の問題なし
    next: reviewers
  - condition: AI特有の問題あり
    next: ai_fix
```
