指定されたlenまでVectorを縮小する。lenが現在の要素数よりも小さい場合、末尾の要素
を削除する。ただし容量は変わらない。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3, 4];
v.truncate(2);
assert_eq!(v, [1, 2]);

let mut u = vector![1, 2, 3];
u.truncate(8);
assert_eq!(u, [1, 2, 3]);

let mut w = vector![1, 2, 3];
w.truncate(0);
assert_eq!(w, []);
```
