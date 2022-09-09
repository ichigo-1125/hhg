指定したインデックスに要素を挿入する。それ以降の要素は右シフトする。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
v.insert(1, 4);
assert_eq!(v, [1, 4, 2, 3]);
v.insert(4, 5);
assert_eq!(v, [1, 4, 2, 3, 5]);
```
