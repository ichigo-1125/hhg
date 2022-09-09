各要素に対して引数で与えられたクロージャを実行し、falseを返す全ての要素を削除す
る。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3, 4];
v.retain(|&x| x % 2 == 0);
assert_eq!(v, [2, 4]);
```
