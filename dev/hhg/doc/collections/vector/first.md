先頭の要素の参照を取得する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![15, 31, 11, 25];
assert_eq!(v.first(), Some(&15));

let u: Vector<i32> = vector![];
assert_eq!(u.first(), None);
```
