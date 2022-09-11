容量を確保する。将来の再確保に備えて余分に容量を確保する可能性がある。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1];
v.reserve(10);
assert!(v.capacity() >= 11);
```
