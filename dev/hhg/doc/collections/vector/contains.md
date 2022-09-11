特定の要素がVectorに含まれているか検索する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![10, 40, 30];
assert!(v.contains(&30));
assert!(!v.contains(&50));
```
