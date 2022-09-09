要素数に合わせて容量を縮小する。

## 使用例

```
use hhg::collections::vector::Vector;

let mut v = Vector::with_capacity(10);
v.extend([1, 2, 3]);
assert_eq!(v.capacity(), 10);
v.shrink_to_fit();
assert_eq!(v.capacity(), 3);
```
