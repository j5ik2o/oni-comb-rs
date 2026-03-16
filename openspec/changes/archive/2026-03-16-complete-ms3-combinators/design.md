# MS3 Combinators 完了 — 設計

## 設計方針

「winnow 並みの速度 + chumsky 並みの使いやすさ」を両立する。

- **速度**: すべて具象コンビネータ型で実装。モノモーフィゼーション → インライン化 → LLVM 最適化が効く
- **使いやすさ**: ParserExt のメソッドチェーンで左から右に読める API
- **一貫性**: 既存の `zip` ファミリーの命名規則を拡張

## 命名設計: zip ファミリー

```
zip        →  両方実行、両方返す     (A, B)
zip_left   →  両方実行、左を返す      A       (= terminated)
zip_right  →  両方実行、右を返す        B     (= preceded)
```

winnow/nom の `preceded`/`terminated` は関数ベースで語順が不自然になるが、メソッドチェーンなら実行順 = 読み順:

```rust
tag("(").zip_right(expr).zip_left(tag(")"))
//  (    → expr を取る   → )   を読み捨てる
```

## 各コンビネータの設計

### ZipLeft<P1, P2> / ZipRight<P1, P2>

```rust
pub struct ZipLeft<P1, P2> { first: P1, second: P2 }
pub struct ZipRight<P1, P2> { first: P1, second: P2 }

// ZipLeft: 両方実行、first の値を返す
impl Parser<I> for ZipLeft<P1, P2> {
    type Output = P1::Output;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        let v = self.first.parse_next(input)?;
        self.second.parse_next(input)?;
        Ok(v)
    }
}

// ZipRight: 両方実行、second の値を返す
impl Parser<I> for ZipRight<P1, P2> {
    type Output = P2::Output;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        self.first.parse_next(input)?;
        self.second.parse_next(input)
    }
}
```

`Zip + Map` よりも効率的: 中間タプルを構築しない。

### between 関数

```rust
pub fn between<I, L, P, R>(left: L, parser: P, right: R)
    -> ZipRight<L, ZipLeft<P, R>>
{
    left.zip_right(parser.zip_left(right))
}
```

新しい型は不要。既存の ZipRight/ZipLeft の合成で実現。

### Many1<P>

```rust
pub struct Many1<P> { parser: P }

impl Parser<I> for Many1<P> {
    type Output = Vec<P::Output>;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        let first = self.parser.parse_next(input)?;
        let mut result = vec![first];
        loop {
            let cp = input.checkpoint();
            match self.parser.parse_next(input) {
                Ok(v) => result.push(v),
                Err(Fail::Backtrack(_)) => { input.reset(cp); break; }
                Err(e) => return Err(e),  // Cut/Incomplete 伝播
            }
        }
        Ok(result)
    }
}
```

Many0 と同じループだが、最初の 1 個が必須。

### SepBy0<P, S> / SepBy1<P, S>

```rust
pub struct SepBy0<P, S> { parser: P, sep: S }

impl Parser<I> for SepBy0<P, S> {
    type Output = Vec<P::Output>;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        let mut result = Vec::new();
        let cp = input.checkpoint();
        match self.parser.parse_next(input) {
            Ok(v) => result.push(v),
            Err(Fail::Backtrack(_)) => { input.reset(cp); return Ok(result); }
            Err(e) => return Err(e),
        }
        loop {
            let cp = input.checkpoint();
            match self.sep.parse_next(input) {
                Ok(_) => {}
                Err(Fail::Backtrack(_)) => { input.reset(cp); break; }
                Err(e) => return Err(e),
            }
            match self.parser.parse_next(input) {
                Ok(v) => result.push(v),
                Err(Fail::Backtrack(_)) => { input.reset(cp); break; }
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}
```

SepBy1 は SepBy0 と同じループだが、最初の要素が Backtrack → Err(Backtrack) を返す。

**sep の後の elem が Backtrack した場合**: checkpoint を sep の前に戻す。これにより trailing separator (e.g. `"a,b,"`) を受け入れず、sep を消費前の状態に巻き戻す。

### ChainL1<P, Op>

```rust
pub struct ChainL1<P, Op> { operand: P, operator: Op }

impl<I, P, Op, F> Parser<I> for ChainL1<P, Op>
where
    P: Parser<I>,
    Op: Parser<I, Output = F, Error = P::Error>,
    F: FnMut(P::Output, P::Output) -> P::Output,
{
    type Output = P::Output;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        let mut acc = self.operand.parse_next(input)?;
        loop {
            let cp = input.checkpoint();
            match self.operator.parse_next(input) {
                Ok(mut f) => {
                    let rhs = self.operand.parse_next(input)?;
                    acc = f(acc, rhs);
                }
                Err(Fail::Backtrack(_)) => { input.reset(cp); break; }
                Err(e) => return Err(e),
            }
        }
        Ok(acc)
    }
}
```

Vec を使わず直接 fold。アロケーションゼロ。

### ChainR1<P, Op>

```rust
pub struct ChainR1<P, Op> { operand: P, operator: Op }

impl Parser<I> for ChainR1<P, Op> {
    type Output = P::Output;
    fn parse_next(&mut self, input: &mut I) -> PResult<..> {
        let first = self.operand.parse_next(input)?;
        let mut operands = vec![first];
        let mut operators = Vec::new();
        loop {
            let cp = input.checkpoint();
            match self.operator.parse_next(input) {
                Ok(f) => {
                    operators.push(f);
                    let v = self.operand.parse_next(input)?;
                    operands.push(v);
                }
                Err(Fail::Backtrack(_)) => { input.reset(cp); break; }
                Err(e) => return Err(e),
            }
        }
        // 右から畳む
        let mut acc = operands.pop().unwrap();
        while let Some(v) = operands.pop() {
            let mut f = operators.pop().unwrap();
            acc = f(v, acc);
        }
        Ok(acc)
    }
}
```

右結合には全要素の収集が必要なため Vec を使う。これは winnow/nom でも同様。

## コスト特性

| コンビネータ | 具象型 | Alloc | Fail 伝播 |
|-------------|--------|-------|-----------|
| zip_left | ZipLeft | zero | Backtrack/Cut そのまま |
| zip_right | ZipRight | zero | Backtrack/Cut そのまま |
| between | ZipRight + ZipLeft | zero | 〃 |
| many1 | Many1 | Vec | 1回目: そのまま、2回目以降: Backtrack で停止、Cut 伝播 |
| sep_by0 | SepBy0 | Vec | elem Backtrack → Ok(vec![])、Cut 伝播 |
| sep_by1 | SepBy1 | Vec | elem Backtrack → Err(Backtrack)、Cut 伝播 |
| chainl1 | ChainL1 | zero | operand Backtrack → Err、operator Backtrack → 停止、Cut 伝播 |
| chainr1 | ChainR1 | Vec | 同上 |

## 実装順序

```
Phase 1: zip_left, zip_right, between    ← 他の全てが使う。テスト容易
Phase 2: many1, sep_by0, sep_by1         ← JSON subset に必要
Phase 3: chainl1, chainr1               ← expression parser に必要
```

## ユースケース検証

### JSON subset

```rust
let value = number.or(string).or(array).or(object);
let array = between(tag("["), value.sep_by0(tag(",")), tag("]"));
let pair = string.zip_left(tag(":")).zip(value);
let object = between(tag("{"), pair.sep_by0(tag(",")), tag("}"));
```

### 四則演算

```rust
let integer = take_while1(|c: char| c.is_ascii_digit())
    .map(|s: &str| s.parse::<i64>().unwrap());
let atom = integer.or(between(tag("("), expr, tag(")")));
let term = atom.chainl1(
    char('*').map(|_| (|a: i64, b| a * b) as fn(i64, i64) -> i64)
        .or(char('/').map(|_| (|a, b| a / b) as fn(i64, i64) -> i64))
);
let expr = term.chainl1(
    char('+').map(|_| (|a: i64, b| a + b) as fn(i64, i64) -> i64)
        .or(char('-').map(|_| (|a, b| a - b) as fn(i64, i64) -> i64))
);
```
