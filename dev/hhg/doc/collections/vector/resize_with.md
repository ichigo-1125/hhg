Vectorをリサイズする。

new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切り捨てら
れる。new_lenが現在の長さよりも大きかった場合、Vectorは拡張され、valueで埋められ
る。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
v.resize_with(5, Default::default);
assert_eq!(v, [1, 2, 3, 0, 0]);

let mut u = vector![];
let mut p = 1;
u.resize_with(4, || { p *= 2; p });
assert_eq!(u, [2, 4, 8, 16]);
```
