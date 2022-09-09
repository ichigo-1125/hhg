Vectorをソートする。アルゴリズムはtimsortベースの適応型反復マージソート。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![-5, 4, 1, -3, 2];
v.sort();
assert!(v == [-5, -3, 1, 2, 4]);
```
