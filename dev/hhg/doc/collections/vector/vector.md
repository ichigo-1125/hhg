ヒープに割り当てられたコンテンツを含む、伸縮可能な配列型。
`Vector<T>`

# 使用例

## `Vector<T>` の生成

[`Vector::new`]、[`vector!`]マクロを用いて新しい[`Vector<T>`]を生成する。ま
た[`Vector::with_capacity()`]を用いれば、あらかじめ必要な容量を指定して生成
することができる。

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v: Vector<i32> = Vector::new();
let u: Vector<i32> = Vector::with_capacity(100);
let s: Vector<i32> = vector![1, 2, 3];
```
