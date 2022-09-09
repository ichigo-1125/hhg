隣り合う要素のうち、等しい要素を削除する。Vectorがソートされていれば、重複する全
ての要素が削除される。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v= vector![1, 2, 2, 2, 3, 2];
v.dedup();
assert_eq!(v, [1, 2, 3, 2]);
```
