指定された範囲を一括で削除し、削除された要素をイテレータとして返す。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
let u: Vector<_> = v.drain(1..).collect();
assert_eq!(v, [1]);
assert_eq!(u, [2, 3]);

v.drain(..);
assert_eq!(v, []);
```
