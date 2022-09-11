容量を取得する。

## 使用例

```
use hhg::collections::Vector;

let u: Vector<i32> = Vector::with_capacity(10);
assert!(u.capacity() >= 10);
```
