指定したインデックスでVectorを分割し、末尾のVectorを返す。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
let mut u = v.split_off(1);
assert_eq!(v, [1]);
assert_eq!(u, [2, 3]);
```
