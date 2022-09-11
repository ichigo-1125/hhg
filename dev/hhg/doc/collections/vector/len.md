要素数を取得する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![15, 31, 11, 25];
assert_eq!(v.len(), 4);

let u: Vector<i32> = Vector::new();
assert_eq!(u.len(), 0);
```
