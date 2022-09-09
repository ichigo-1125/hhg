与えられたクロージャによりVectorをソートする。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut floats = vector![5f64, 4.0, 1.0, 3.0, 2.0];
floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
```
