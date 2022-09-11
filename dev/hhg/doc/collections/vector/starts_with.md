Vectorの先頭の要素がneedleで始まるかどうかを調べる。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![10, 40, 30];
assert!(v.starts_with(&[10]));
assert!(v.starts_with(&[10, 40]));
assert!(!v.starts_with(&[50]));
assert!(!v.starts_with(&[10, 50]));

let u = vector![10, 40, 30];
assert!(u.starts_with(&[]));
let s: Vector<u8> = vector![];
assert!(s.starts_with(&[]));
```
