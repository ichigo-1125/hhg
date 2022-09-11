キー抽出関数によりVectorをソートする。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![-5i32, 4, 1, -3, 2];
v.sort_by_key(|k| k.abs());
assert!(v == [1, 2, -3, 4, -5]);
```
