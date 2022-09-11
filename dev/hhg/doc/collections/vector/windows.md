与えられたsizeごとに連続するスライスのイテレータを返す。ウィンドウは重なる。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector!['r', 'u', 's', 't'];
let mut iter = v.windows(2);
assert_eq!(iter.next().unwrap(), &['r', 'u']);
assert_eq!(iter.next().unwrap(), &['u', 's']);
assert_eq!(iter.next().unwrap(), &['s', 't']);
assert!(iter.next().is_none());

let u = vector!['f', 'o', 'o'];
let mut iter = u.windows(4);
assert!(iter.next().is_none());
```
