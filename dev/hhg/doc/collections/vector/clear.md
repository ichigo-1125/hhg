全ての要素を削除する。ただし容量は変わらない。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
v.clear();
assert!(v.is_empty());
```
