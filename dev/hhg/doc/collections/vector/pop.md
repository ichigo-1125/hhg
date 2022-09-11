末尾の要素を取得する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
assert_eq!(v.pop(), Some(3));
assert_eq!(v, [1, 2]);
```
