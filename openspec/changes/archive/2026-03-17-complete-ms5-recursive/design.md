# MS5 Recursive — 設計

## API

```rust
let expr = recursive(|expr| {
    let atom = integer().or(between(tag("("), expr, tag(")")));
    let term = atom.chainl1(mul_op());
    term.chainl1(add_op())
});
```

`recursive(f)` は:
1. 空の `Recursive` を作成（inner = None）
2. `f` にクローン可能な再帰参照を渡す
3. `f` の戻り値を `Box<dyn Parser>` にして inner にセット
4. `Recursive` を返す

## 内部構造

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct RecursiveInner<I: Input> {
    inner: RefCell<Option<Box<dyn Parser<I, Output = ???, Error = ???>>>>,
}

pub struct Recursive<I: Input> {
    shared: Rc<RecursiveInner<I>>,
}
```

### 型パラメータの問題

`Box<dyn Parser<I, Output = O, Error = E>>` にするためには、Output と Error の型を `Recursive` の型パラメータに持つ必要がある。

```rust
pub struct Recursive<I: Input, O, E> {
    shared: Rc<RecursiveInner<I, O, E>>,
}

struct RecursiveInner<I: Input, O, E> {
    inner: RefCell<Option<Box<dyn Parser<I, Output = O, Error = E>>>>,
}
```

### parse_next の実装

```rust
impl<I, O, E> Parser<I> for Recursive<I, O, E>
where I: Input
{
    type Output = O;
    type Error = E;

    fn parse_next(&mut self, input: &mut I) -> PResult<O, E> {
        self.shared.inner.borrow_mut().as_mut()
            .expect("recursive parser not initialized")
            .parse_next(input)
    }
}
```

### Clone の実装

クロージャ内で `expr` を複数箇所で使うために `Clone` が必要。`Rc` のクローンなのでゼロコスト。

```rust
impl<I: Input, O, E> Clone for Recursive<I, O, E> {
    fn clone(&self) -> Self {
        Recursive { shared: Rc::clone(&self.shared) }
    }
}
```

### recursive() 関数

```rust
pub fn recursive<I, O, E, F, P>(f: F) -> Recursive<I, O, E>
where
    I: Input,
    F: FnOnce(Recursive<I, O, E>) -> P,
    P: Parser<I, Output = O, Error = E> + 'static,
{
    let rec = Recursive {
        shared: Rc::new(RecursiveInner {
            inner: RefCell::new(None),
        }),
    };
    let parser = f(rec.clone());
    *rec.shared.inner.borrow_mut() = Some(Box::new(parser));
    rec
}
```

## StrInput のライフタイム問題

`Box<dyn Parser<StrInput<'a>, Output = O, Error = E>>` は `'a` に依存する。
`recursive` を `StrInput<'a>` で使う場合、`'static` 制約が問題になる可能性。

解決策: `P: Parser<I, ...> + 'static` の代わりに `P: Parser<I, ...> + 'a` にする。
ただし `Rc<RefCell<...>>` の中の `dyn Parser` にもライフタイムが必要。

```rust
struct RecursiveInner<'a, O, E> {
    inner: RefCell<Option<Box<dyn Parser<StrInput<'a>, Output = O, Error = E> + 'a>>>,
}
```

これは `StrInput` 専用になる。汎用にするには GAT か、I を固定するか。
**当面は `StrInput` 専用で実装し、後で汎化**する方針。

## Fail 意味論

`Recursive` は内部パーサーの Fail をそのまま伝播する（透過的）。

## コスト

| 操作 | コスト |
|------|--------|
| 構築時 | Rc 1個 + Box 1個（1回だけ） |
| parse_next | RefCell::borrow_mut + Box の間接呼び出し |

Rc + RefCell のオーバーヘッドは再帰の結び目だけで発生。非再帰部分は具象型のまま。
