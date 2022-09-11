Vectorの末尾の要素がneedleで終わるかどうかを調べる。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![10, 40, 30];
assert!(v.ends_with(&[30]));
assert!(v.ends_with(&[40, 30]));
assert!(!v.ends_with(&[50]));
assert!(!v.ends_with(&[50, 30]));

let u = vector![10, 40, 30];
assert!(u.ends_with(&[]));
let s: Vector<u8> = vector![];
assert!(s.ends_with(&[]));
```
