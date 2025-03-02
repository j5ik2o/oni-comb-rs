# ParseErrorの設計改善提案

## 現状の問題点

現在、`failed_lazy_static`関数を使用したテストでライフタイムエラーが発生しています。問題の根本原因は以下の通りです：

1. `ParseError<'a, I>`構造体は`'a`ライフタイムを持ち、内部に`&'a [I]`という参照を保持しています  
2. `failed_lazy_static`関数は、`Fn() -> (ParseError<'a, I>, CommittedStatus)`という関数を引数に取ります  
3. テストコードでは、クロージャ内でローカル変数`input`への参照を含む`ParseError`を作成しています  
4. クロージャのライフタイム`'1`が、返される`ParseError`のライフタイム`'2`より短いため、コンパイルエラーが発生しています

```rust
let p: static_parser::StaticParser<'_, char, char> = failed_lazy_static(move || {
  counter_clone.set(counter_clone.get() + 1);
  (
    ParseError::of_mismatch(&input, 0, 0, format!("error message: {}", counter_clone.get())),
    CommittedStatus::Uncommitted,
  )
});
```

## 設計改善案

### 案1: 所有権ベースのParseError設計

`ParseError`を参照ではなく所有権ベースに変更します：

```rust
pub enum ParseError<I> {
  Mismatch {
    input: Vec<I>,  // &'a [I]の代わりにVec<I>を使用
    offset: usize,
    length: usize,
    message: String,
  },
  // 他のバリアントも同様に変更
}
```

**メリット**:
- ライフタイム問題が解消される
- 所有権の管理が単純になる

**デメリット**:
- 入力データのコピーが必要になり、パフォーマンスに影響する可能性がある
- 既存コードの広範な変更が必要

### 案2: 二重構造のParseError設計

参照を持つバージョンと所有権を持つバージョンの両方を提供します：

```rust
pub enum ParseError<'a, I> {
  // 現状のまま（参照ベース）
}

pub enum OwnedParseError<I> {
  // 所有権ベースのバージョン
}

// 変換メソッドを提供
impl<'a, I: Clone> ParseError<'a, I> {
  pub fn to_owned(&self) -> OwnedParseError<I> {
    // 変換ロジック
  }
}

impl<I> OwnedParseError<I> {
  pub fn as_ref<'a>(&'a self) -> ParseError<'a, I> {
    // 変換ロジック
  }
}
```

**メリット**:
- 既存のAPIを維持しながら、所有権ベースの選択肢も提供できる
- 用途に応じて適切な型を選択できる

**デメリット**:
- コードの複雑性が増す
- 二つの型間の変換が必要になる場合がある

### 案3: Cow（Clone-on-Write）を活用した設計

`std::borrow::Cow`を使用して、参照と所有権の両方に対応します。以下は実装例です：

```rust
use std::borrow::Cow;

pub enum ParseError<'a, I: Clone + 'a> {
  Mismatch {
    input: Cow<'a, [I]>,
    offset: usize,
    length: usize,
    message: String,
  },
  // 他のバリアントも同様に変更
}
```

**メリット**:
- 必要な場合のみクローンが行われるため、効率的
- 単一の型で参照と所有権の両方のケースに対応できる

**デメリット**:
- `I: Clone`制約が追加される
- 既存コードの変更が必要

### 案4: 問題のある関数の代替設計

`failed_lazy_static`関数を修正または代替関数を提供します：

```rust
// 代替関数: 所有権ベースのエラーを生成する
pub fn failed_lazy_owned_static<'a, I: Clone, A, F>(f: F) -> StaticParser<'a, I, A>
where
  F: Fn() -> (OwnedParseError<I>, CommittedStatus) + 'a,
  I: 'a,
  A: 'a,
{
  // 実装
}

// または、エラーメッセージのみを生成する関数
pub fn failed_with_message_static<'a, I, A, F>(f: F) -> StaticParser<'a, I, A>
where
  F: Fn() -> (String, CommittedStatus) + 'a,
  I: 'a + Clone,
  A: 'a,
{
  // 内部でParseErrorを生成する実装
}
```

**メリット**:
- 既存の`ParseError`構造を変更せずに問題を解決できる
- 特定のユースケースに最適化された関数を提供できる

**デメリット**:
- APIの一貫性が低下する可能性がある
- 追加の関数が必要になる

## 推奨案

案3（Cowを活用した設計）を推奨します。この方法は以下の理由で最適と考えられます：

1. 既存のAPIの意味論を大きく変えることなく、ライフタイム問題を解決できる  
2. パフォーマンスと柔軟性のバランスが良い  
3. Rustの標準ライブラリのパターンに従っている  
4. 実装の変更が比較的局所的に抑えられる

## 実装計画

1. `ParseError`構造体を`Cow`を使用するように修正  
2. 関連するメソッドを更新  
3. テストを修正して新しい設計をテスト  
4. ドキュメントを更新

この変更により、`failed_lazy_static`関数を廃止する必要はなく、テストコードも期待通りに動作するようになります。