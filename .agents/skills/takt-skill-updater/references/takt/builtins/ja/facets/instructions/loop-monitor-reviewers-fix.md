reviewers → fix のループが {cycle_count} 回繰り返されました。

Report Directory 内の最新レビューレポートを確認し、
このループが健全（収束傾向）か非生産的（発散・振動）かを判断してください。

**判断基準:**
- 各サイクルで new / reopened の指摘件数が減少しているか
- 同じ family_tag の指摘が繰り返されていないか（persists が増えていないか）
- 修正が実際にコードに反映されているか
