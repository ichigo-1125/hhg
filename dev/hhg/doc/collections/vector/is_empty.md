Vectorが空であるかを確認する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![15, 31, 11, 25];
assert_eq!(v.is_empty(), false);

let u: Vector<i32> = Vector::new();
assert_eq!(u.is_empty(), true);
```
