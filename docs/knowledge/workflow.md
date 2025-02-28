# ワークフロールール

## 基本的な開発プロセス

プロジェクトでの作業は以下のプロセスに従って行います：

1. **コード編集**
   - コードの変更は、明確な目的を持って行う
   - コーディング規約に従う（[programming.md](programming.md) 参照）
   - 変更は小さく、理解しやすい単位で行う

2. **テスト実行**
   - コードを編集したら必ず `cargo test` を実行する
   - すべてのテストがパスすることを確認する
   - 必要に応じて新しいテストを追加する

3. **適切な粒度での git commit**
   - 論理的に関連する変更をまとめてコミットする
   - コミットメッセージは[common.md](common.md)のバージョン管理ルールに従う
   - 例: `fix: correct rustdoc formatting in gen module`
   - 例: `feat: add new generator for custom types`
   - 例: `docs: update workflow documentation`

4. **ナレッジの文書化**
   - 作業中に獲得した知識は[common.md](common.md)のナレッジ管理ルールに従って記録する

## 具体的なタスク実行例

### コードスタイルの修正

```
1. コード編集: rustdoc のフォーマットを統一
2. テスト実行: cargo test でテストが通ることを確認
3. git commit: "style: standardize rustdoc format across all modules"
4. ナレッジ反映: docs/knowledge/programming.md に rustdoc の書式ガイドラインを追加
```

### 新機能の追加

```
1. コード編集: 新しい機能を実装
2. テスト実行: 新機能のテストを追加し、すべてのテストが通ることを確認
3. git commit: "feat: add support for custom type generators"
4. ナレッジ反映: 必要に応じて docs/ 以下に新機能の使用方法や設計意図を記録
```

### バグ修正

```
1. コード編集: バグを修正
2. テスト実行: バグを再現するテストを追加し、修正後にパスすることを確認
3. git commit: "fix: resolve integer overflow in large number generation"
4. ナレッジ反映: バグの原因や解決策に関する知見を docs/ に記録
```

## 重要なポイント

- 各ステップは順番に実行し、特にテスト実行は必ず行う
- コミットは小さく、意味のある単位で行う
- ドキュメントは常に最新の状態を保つ
- 獲得した知識は個人のものではなく、プロジェクト全体の資産として共有する