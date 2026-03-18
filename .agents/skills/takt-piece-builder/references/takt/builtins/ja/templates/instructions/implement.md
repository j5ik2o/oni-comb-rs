# implement — 実装 instruction テンプレート

> **用途**: コーディング・テスト実行
> **使用エージェント**: coder
> **レポート**: Scope + Decisions（フォーマットをテンプレート内に埋め込み）

---

## テンプレート

```
{カスタマイズ: 参照元ムーブメントに応じて変更}
planムーブメントで立てた計画に従って実装してください。

**参照するレポート:**
- 計画: {report:plan.md}
{カスタマイズ: architect ムーブメントがある場合に追加}
- 設計: {report:architecture.md}（存在する場合）

Piece Contextに示されたReport Directory内のファイルのみ参照してください。
他のレポートディレクトリは検索/参照しないでください。

{カスタマイズ: architect がある場合に追加}
**重要:** 設計判断はせず、architectムーブメントで決定された設計に従ってください。
不明点や設計の変更が必要な場合は報告してください。

**重要**: 実装と同時に単体テストを追加してください。
- 新規作成したクラス・関数には単体テストを追加
- 既存コードを変更した場合は該当するテストを更新
- テストファイルの配置: プロジェクトの規約に従う
- テスト実行は必須。実装完了後、必ずテストを実行して結果を確認

**Scope出力契約（実装開始時に作成）:**
```markdown
# 変更スコープ宣言

## タスク
{タスクの1行要約}

## 変更予定
| 種別 | ファイル |
|------|---------|
| 作成 | `src/example.ts` |
| 変更 | `src/routes.ts` |

## 推定規模
Small / Medium / Large

## 影響範囲
- {影響するモジュールや機能}
```

**Decisions出力契約（実装完了時、決定がある場合のみ）:**
```markdown
# 決定ログ

## 1. {決定内容}
- **背景**: {なぜ決定が必要だったか}
- **検討した選択肢**: {選択肢リスト}
- **理由**: {選んだ理由}
```

**必須出力（見出しを含める）**
## 作業結果
- {実施内容の要約}
## 変更内容
- {変更内容の要約}
## テスト結果
- {実行コマンドと結果}
```

---

## 典型的な rules

```yaml
rules:
  - condition: 実装完了
    next: {ai_review or reviewers}
  - condition: 実装未着手（レポートのみ）
    next: {ai_review or reviewers}
  - condition: 判断できない、情報不足
    next: {ai_review or reviewers}
  - condition: ユーザー入力が必要
    next: implement
    requires_user_input: true
    interactive_only: true
```

---

## レポート設定

```yaml
report:
  - Scope: coder-scope.md
  - Decisions: coder-decisions.md
```

**注意**: レポートファイル名に連番を付けない。
`02-coder-scope.md` ではなく `coder-scope.md` とする。
連番はピース構造に依存するため、テンプレートの再利用を妨げる。
