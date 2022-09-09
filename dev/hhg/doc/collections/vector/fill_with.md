クロージャの返り値でVectorを埋める。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut buf = vector![1; 10];
buf.fill_with(Default::default);
assert_eq!(buf, vec![0; 10]);
```
