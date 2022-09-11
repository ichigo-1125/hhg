指定したインデックスの要素の参照を取得する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![15, 31, 11, 25];
assert_eq!(v.get(2), Some(&11));
assert_eq!(v.get(1..3), Some(&[31, 11][..]));
assert_eq!(v.get(8), None);
assert_eq!(v.get(1..8), None);
```
