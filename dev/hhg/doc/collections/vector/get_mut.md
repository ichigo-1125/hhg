指定したインデックスの要素の可変参照を取得する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![15, 31, 11, 25];
assert_eq!(v.get_mut(2), Some(&mut 11));
assert_eq!(v.get_mut(1..3), Some(&mut [31, 11][..]));
assert_eq!(v.get_mut(8), None);
assert_eq!(v.get_mut(1..8), None);
```
