別のVectorを末尾に移動する。otherは空になる。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
let mut u = vector![4, 5, 6];
v.append(&mut u);
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
assert_eq!(u, []);
```
