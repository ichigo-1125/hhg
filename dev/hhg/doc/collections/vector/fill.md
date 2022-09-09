値でVectorを埋める。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut buf = vector![0; 10];
buf.fill(1);
assert_eq!(buf, vec![1; 10]);
```
