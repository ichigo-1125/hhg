splitを末尾から実行する。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v = vector![11, 22, 33, 0, 44, 55];
let mut iter = v.rsplit(|num| *num == 0);

assert_eq!(iter.next().unwrap(), &[44, 55]);
assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
assert_eq!(iter.next(), None);
```
