Vectorをリサイズする。

new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切り捨てら
れる。new_lenが現在の長さよりも大きかった場合、Vectorは拡張され、valueで埋められ
る。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![15, 31, 11, 25];

v.resize(6, 20);
assert_eq!(v, [15, 31, 11, 25, 20, 20]);

v.resize(3, 20);
assert_eq!(v, [15, 31, 11]);
```
